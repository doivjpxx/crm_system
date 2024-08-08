-- Add up migration script here
CREATE TABLE plans (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(255) NOT NULL,
  description TEXT NOT NULL,
  price BIGINT NOT NULL,
  is_active BOOLEAN DEFAULT TRUE,
  tags TEXT[] DEFAULT '{}',
  trial_days INT DEFAULT 0,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO plans (id, name, description, price, is_active, tags, trial_days, created_at, updated_at) VALUES
('0137ac8e-ca1e-4446-a017-40b4cdbbe92f', 'Premium', 'Gói cao cấp', 49000, TRUE, '{}', 30, '2024-08-07 16:11:50.410405', '2024-08-07 16:11:50.410405'),
('13cafdb3-a88d-4987-8119-0470caebd56c', 'Basic', 'Gói cơ bản', 19000, TRUE, '{}', 30, '2024-08-07 16:11:01.287972', '2024-08-07 16:11:01.287972'),
('24a085b1-f12f-40f6-820a-c085c7d10f60', 'Free', 'Gói miễn phí', 0, TRUE, '{}', -1, '2024-08-07 16:10:31.776209', '2024-08-07 16:10:31.776209'),
('aab372ed-af15-4508-bd27-4924ce8147d7', 'Advance', 'Gói nâng cao', 29000, TRUE, '{}', 30, '2024-08-07 16:11:34.404398', '2024-08-07 16:11:34.404398');