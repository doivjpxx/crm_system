-- Add up migration script here
CREATE TABLE payments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    subscription_id UUID REFERENCES subscriptions(id) ON DELETE CASCADE,
    amount BIGINT NOT NULL,
    payment_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    payment_method VARCHAR(50)
);

INSERT INTO payments (id, subscription_id, amount, payment_date, payment_method) VALUES
('dd2293d0-54b2-495a-8619-74298ef4253f', 'ebaee4f4-fc3d-4e6e-94d6-2728db9cdd14', -1, '2024-08-07 16:15:28.85862', 'BANK_TRANSFER');