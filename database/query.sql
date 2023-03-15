SELECT *
FROM storage_object
WHERE created_at > CURRENT_DATE - INTERVAL '7 days';
