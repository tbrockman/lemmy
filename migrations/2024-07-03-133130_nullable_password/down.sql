-- This file should undo anything in `up.sql`
ALTER TABLE local_user ALTER COLUMN password_encrypted SET NOT NULL
UPDATE local_user SET password_encrypted = substr(md5(random()::text), 0, 33) WHERE password_encrypted IS NULL