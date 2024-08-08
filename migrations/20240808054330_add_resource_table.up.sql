-- Add up migration script here
-- Resource là các gói con của Plan, mỗi Plan có thể chứa nhiều Resource
-- Ví dụ: Plan basic có thể tạo tối đa 5 group, Plan advance có thể tạo tối đa 10 group, Plan premium có thể tạo tối đa 20 group
-- Ví dụ: Plan basic có thể tạo tối đa 2 role, Plan advance có thể tạo tối đa 5 role, Plan premium có thể tạo tối đa 10 role
-- Resource sẽ chứa thông tin về số lượng hiện tại và số lượng tối đa của một Plan
CREATE TABLE resources (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    plan_id UUID NOT NULL,
    max BIGINT NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (plan_id) REFERENCES plans(id)
);

INSERT INTO resources (plan_id, max, name, description, created_at, updated_at) VALUES
('24a085b1-f12f-40f6-820a-c085c7d10f60', 3, 'user_free_resource', 'Tạo người dùng cho gói Free', '2024-08-08 05:43:30.000000', '2024-08-08 05:43:30.000000'),
('24a085b1-f12f-40f6-820a-c085c7d10f60', 2, 'role_free_resource', 'Tạo phân quyền cho gói Free', '2024-08-08 05:43:30.000000', '2024-08-08 05:43:30.000000'),
('13cafdb3-a88d-4987-8119-0470caebd56c', 10, 'user_basic_resource', 'Tạo người dùng cho gói Basic', '2024-08-08 05:43:30.000000', '2024-08-08 05:43:30.000000'),
('13cafdb3-a88d-4987-8119-0470caebd56c', 4, 'role_basic_resource', 'Tạo phân quyền cho gói Basic', '2024-08-08 05:43:30.000000', '2024-08-08 05:43:30.000000'),
('aab372ed-af15-4508-bd27-4924ce8147d7', 20, 'user_advance_resource', 'Tạo người dùng cho gói Advance', '2024-08-08 05:43:30.000000', '2024-08-08 05:43:30.000000'),
('aab372ed-af15-4508-bd27-4924ce8147d7', 6, 'role_advance_resource', 'Tạo phân quyền cho gói Advance', '2024-08-08 05:43:30.000000', '2024-08-08 05:43:30.000000'),
('0137ac8e-ca1e-4446-a017-40b4cdbbe92f', 50, 'user_premium_resource', 'Tạo người dùng cho gói Premium', '2024-08-08 05:43:30.000000', '2024-08-08 05:43:30.000000'),
('0137ac8e-ca1e-4446-a017-40b4cdbbe92f', 8, 'role_premium_resource', 'Tạo phân quyền cho gói Premium', '2024-08-08 05:43:30.000000', '2024-08-08 05:43:30.000000');