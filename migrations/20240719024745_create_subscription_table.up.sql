-- Add up migration script here
CREATE TABLE subscriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    plan_id UUID REFERENCES plans(id) ON DELETE CASCADE,
    start_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    end_date TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE,
    trial_start_date TIMESTAMP,
    trial_end_date TIMESTAMP
);

INSERT INTO subscriptions (id, user_id, plan_id, start_date, end_date, is_active, trial_start_date, trial_end_date) VALUES
('ebaee4f4-fc3d-4e6e-94d6-2728db9cdd14', 'cd187045-8b42-4b8a-8913-abaf62ea72c9', '0137ac8e-ca1e-4446-a017-40b4cdbbe92f', '2024-08-07 16:15:28.85862', '2024-09-06 16:15:28.85862', TRUE, NULL, NULL);