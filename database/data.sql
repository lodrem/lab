INSERT INTO storage_object (id, "key", "type", "storage", "path", created_at)
VALUES (gen_random_uuid(), 'foo', 'avatar', 'aws-s3', '/avatar/foo.jpg', NOW()),
       (gen_random_uuid(), 'bar', 'avatar', 'aws-s3', '/avatar/bar.jpg', NOW())
RETURNING *;
