INSERT INTO
    roles (name)
VALUES
    ('Admin'),
    ('User')
ON CONFLICT DO NOTHING;

-- Passw0rd
INSERT INTO
    users (name, email, password_hash, role_id)
SELECT
    'Eleazar Fig',
    'eleazar.fig@example.com',
    '$2b$12$qeVsJInZ/3gu08qe6UgYwO5rC9Jz1xwk92E10lYsPhRiVmc4YqF.a',
    role_id
FROM
    roles
WHERE
    name LIKE 'Admin';
