mod config;
mod unstructured;

use db::queries;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up tracing/logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Load configuration
    let config = config::Config::new();
    dbg!(&config);

    // Create connection pool and get a client
    let pool = db::create_pool(&config.app_database_url);
    let mut client = pool.get().await?;

    loop {
        // Get all unprocessed documents
        let unprocessed_docs = queries::documents::unprocessed_documents()
            .bind(&client)
            .all()
            .await?;

        for document in unprocessed_docs {
            let dataset = queries::datasets::pipeline_dataset()
                .bind(&client, &document.dataset_id)
                .one()
                .await?;

            let structured_data = crate::unstructured::call_unstructured_api(
                document.content,
                &document.file_name,
                dataset.combine_under_n_chars as u32,
                dataset.new_after_n_chars as u32,
                dataset.multipage_sections,
                &config.unstructured_endpoint,
            )
            .await;

            match structured_data {
                Ok(structured_data) => {
                    let transaction = client.transaction().await?;

                    if let Some(key) = &config.customer_key {
                        transaction
                            .execute(&format!("SET LOCAL encryption.root_key = '{}'", key), &[])
                            .await?;
                    }

                    for text in structured_data {
                        transaction
                            .execute(
                                r#"
                                INSERT INTO chunks (
                                    document_id,
                                    page_number,
                                    text
                                ) 
                                VALUES 
                                    ($1, $2, encrypt_text($3))
                                "#,
                                &[
                                    &document.id,
                                    &text.metadata.page_number.unwrap_or(0),
                                    &text.text,
                                ],
                            )
                            .await?;
                    }

                    // 5) Commit the transaction
                    transaction.commit().await?;
                }
                Err(error) => {
                    let error_msg = format!("Not able to parse document {}", error);
                    // If document parse fails, you might update your DB outside of that transaction
                    queries::documents::fail_document()
                        .bind(&client, &error_msg, &document.id)
                        .await?;

                    tracing::error!(%error_msg);
                }
            }
        }

        // Next, process any chunks that are unprocessed (embedding step).
        // This section does not need the SET LOCAL if you are not encrypting,
        // so you can do it outside any custom transaction or use a new one if preferred.
        loop {
            let unprocessed = queries::chunks::unprocessed_chunks()
                .bind(&client)
                .all()
                .await?;

            if unprocessed.is_empty() {
                break;
            }

            if let Some(embedding) = unprocessed.first() {
                // (Optional) you could also wrap each embedding update in its own transaction
                // if you want. For brevity, we’ll just do it using the client directly here.

                let embeddings = embeddings_api::get_embeddings(
                    &embedding.text,
                    &embedding.base_url,
                    &embedding.model,
                    &embedding.api_key,
                )
                .await;

                if let Ok(embeddings) = embeddings {
                    let embedding_data = pgvector::Vector::from(embeddings);

                    client
                        .execute(
                            r#"
                            UPDATE chunks 
                            SET (processed, embeddings) = (TRUE, $1)
                            WHERE id = $2
                            "#,
                            &[&embedding_data, &embedding.id],
                        )
                        .await?;

                    tracing::info!("Processing embedding id {:?}", embedding.id);
                } else {
                    tracing::info!("Failed to process embedding id {:?}", embedding.id);
                    client
                        .execute(
                            r#"
                            UPDATE chunks 
                            SET processed = TRUE
                            WHERE id = $1
                            "#,
                            &[&embedding.id],
                        )
                        .await?;
                }
            }
        }

        // Run this loop every 5 seconds
        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    }
}
