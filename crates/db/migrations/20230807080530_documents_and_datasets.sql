-- migrate:up

CREATE EXTENSION IF NOT EXISTS vector;

CREATE TYPE chunking_strategy AS ENUM (
    'ByTitle'
);
COMMENT ON TYPE role IS 'Chunking strategies as supported by unstructured';

CREATE TABLE datasets (
    id int GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY, 
    team_id INT NOT NULL, 
    embeddings_model_id INT NOT NULL,
    visibility visibility NOT NULL,
    name VARCHAR NOT NULL, 
    chunking_strategy chunking_strategy NOT NULL,
    combine_under_n_chars INT NOT NULL,
    new_after_n_chars INT NOT NULL,
    multipage_sections BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT FK_team FOREIGN KEY(team_id)
        REFERENCES teams(id) ON DELETE CASCADE,

    CONSTRAINT FK_user FOREIGN KEY(created_by)
        REFERENCES users(id) ON DELETE CASCADE,

    CONSTRAINT FK_model FOREIGN KEY(embeddings_model_id)
        REFERENCES models(id) ON DELETE CASCADE
);

CREATE TABLE documents (
    id int GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY, 
    dataset_id INT NOT NULL, 
    file_name VARCHAR NOT NULL, 
    file_type VARCHAR, 
    failure_reason VARCHAR, 
    content BYTEA NOT NULL,
    content_size INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT FK_dataset FOREIGN KEY(dataset_id)
        REFERENCES datasets(id) ON DELETE CASCADE
);

CREATE TABLE chunks (
    id int GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY, 
    document_id INT NOT NULL, 
    text VARCHAR NOT NULL, 
    page_number INT NOT NULL, 
    embeddings vector, 
    processed BOOL NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT FK_document FOREIGN KEY(document_id)
        REFERENCES documents(id) ON DELETE CASCADE
);

-- Give access to the application user.
GRANT SELECT, INSERT, UPDATE, DELETE ON documents, chunks, datasets TO bionic_application;
GRANT USAGE, SELECT ON documents_id_seq, chunks_id_seq, datasets_id_seq TO bionic_application;

-- Give access to the readonly user
GRANT SELECT, INSERT, UPDATE, DELETE ON documents, chunks, datasets TO bionic_readonly;
GRANT USAGE, SELECT ON documents_id_seq, chunks_id_seq, datasets_id_seq TO bionic_readonly;

-- migrate:down
DROP TABLE chunks;
DROP TABLE documents;
DROP TABLE datasets;
DROP TYPE chunking_strategy;
DROP EXTENSION vector;

