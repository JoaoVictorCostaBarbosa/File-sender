-- Add migration script here
CREATE TABLE glb_models (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    model_data BYTEA NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
