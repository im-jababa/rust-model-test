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
    (? IS NULL OR `name` LIKE CONCAT('%', ?, '%'))
    AND
    (? IS NULL OR `provider` IN ({}))
    AND
    (? IS NULL OR (`email` IS NOT NULL) = ?)
ORDER BY
    CASE
        WHEN ? IS NULL THEN 0
        WHEN `name` = ? THEN 0
        ELSE 1
    END ASC,
    CASE
        WHEN ? IS NULL THEN 0
        ELSE LOCATE(?, `name`)
    END ASC,
    CHAR_LENGTH(`name`) ASC,
    `created_at` DESC
