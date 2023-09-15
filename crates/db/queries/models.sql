--: Model()

--! models : Model
SELECT
    id,
    name,
    base_url,
    billion_parameters,
    context_size_bytes,
    created_at,
    updated_at
FROM 
    models
ORDER BY updated_at;

--! model : Model
SELECT
    id,
    name,
    base_url,
    billion_parameters,
    context_size_bytes,
    created_at,
    updated_at
FROM 
    models
WHERE
    id = :model_id
ORDER BY updated_at;

--! insert
INSERT INTO models (
    name,
    organisation_id,
    base_url,
    billion_parameters,
    context_size_bytes
)
VALUES(
    :name, :organisation_id, :base_url, :billion_parameters, :context_size_bytes
);

--! update
UPDATE 
    models 
SET 
    name = :name, 
    base_url = :base_url,
    billion_parameters = :billion_parameters,
    context_size_bytes = :context_size_bytes
WHERE
    id = :id;