

INSERT INTO user (kind, nickname, username, active, status, create_at, update_at, deleted)
VALUES (1, 'yehun', 'yehun', 0, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0);
BEGIN TRANSACTION;
INSERT INTO user_password (user_id, password, create_at, update_at, deleted)
VALUES (last_insert_rowid(), md5('123456'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0);

INSERT INTO user_email (user_id, email, create_at, update_at, deleted)
VALUES (last_insert_rowid(), 'yehunhk@163.com', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0);

INSERT INTO user_phone (user_id, phone, create_at, update_at, deleted)
VALUES (last_insert_rowid(), '13800000000', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0);
COMMIT;