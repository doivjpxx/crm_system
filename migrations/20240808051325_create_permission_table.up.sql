-- Add up migration script here
CREATE TABLE permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO permissions (id, name, description, created_at) VALUES
('5288da3a-e937-4ce3-92fc-ba4808bbffd2', 'CREATE', 'Create permission', '2024-08-07 16:15:28.85862'),
('801a2a2e-194d-41dd-b0e2-512428105a33', 'UPDATE', 'Update permission', '2024-08-07 16:15:28.85862'),
('554a358f-adb0-4705-9874-6f54a3924642', 'DELETE', 'Delete permission', '2024-08-07 16:15:28.85862'),
('70699277-b0be-4227-8f3c-f1589e69499d', 'READ', 'Read permission', '2024-08-07 16:15:28.85862');