SELECT
    `pk`,
    `id`,
    `provider`,
    `sub`,
    `name`,
    `email`,
    `created_at`,
    `updated_at`
FROM
    `user`
WHERE
    `sub` = ?
