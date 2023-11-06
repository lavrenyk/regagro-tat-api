-- 08/10/2023.
ALTER DATABASE `regagro_3_0` DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci;
DROP TABLE IF EXISTS `aminals`;
CREATE TABLE `animals` (
    `id` int NOT NULL AUTO_INCREMENT,
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP,
    `birth_date` TIMESTAMP,
    `birth_date_edited` TINYINT,
    `is_tribal` TINYINT,
    `tribal_certificate` VARCHAR(255),
    `kind_id` BIGINT,
    `suit_id` BIGINT,
    `breed_id` BIGINT,
    `registration_ground_id` BIGINT,
    `gender` TINYINT,
    `keep_type_id` BIGINT,
    `keep_purpose_id` BIGINT,
    `keep_place_id` BIGINT,
    `enterprise_id` BIGINT,
    `count` INT,
    `name` VARCHAR(64),
    `deleted_at` TIMESTAMP,
    `user_id` BIGINT,
    `is_offspring` TINYINT,
    `is_insemination` TINYINT,
    `is_castrated` TINYINT,
    `product_direction_id` BIGINT,
    `number` VARCHAR(255),
    `is_group` TINYINT,
    `guid` CHAR(36),
    `animal_parent_id` BIGINT,
    `is_super_group` TINYINT,
    `is_mobile` TINYINT,
    `first_marker_date` TIMESTAMP,
    `cross_id` BIGINT,
    `from_group_id` BIGINT,
    PRIMARY KEY (`id`)
);
DROP TABLE IF EXISTS `enterprises`;
CREATE TABLE `enterprises` (
    `id` int NOT NULL AUTO_INCREMENT,
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP,
    `name` VARCHAR(64),
    `uuid` VARCHAR(36),
    `longitude` DECIMAL(10, 7),
    `latitude` DECIMAL(9, 7),
    `owner_id` BIGINT(20),
    `enterprise_type_id` BIGINT(20),
    `user_id` BIGINT(20),
    `service_area_id` BIGINT(20),
    PRIMARY KEY (`id`)
);
DROP TABLE IF EXISTS `enterprise_addresses`;
CREATE TABLE `enterprise_addresses` (
    `id` int NOT NULL AUTO_INCREMENT,
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP,
    `full_address` VARCHAR(255),
    `postal_code` VARCHAR(30),
    `country_code` VARCHAR(36),
    `region_code` VARCHAR(36),
    `district_code` VARCHAR(36),
    `locality_code` VARCHAR(36),
    `sub_locality_code` VARCHAR(36),
    `pos_council_code` VARCHAR(36),
    `street_code` VARCHAR(36),
    `house` VARCHAR(45),
    `block` VARCHAR(45),
    `flat` VARCHAR(45),
    `enterprise_id` BIGINT(20),
    `address_type_id` BIGINT(20),
    `comment` TEXT,
    `plan_structure_code` CHAR(36),
    PRIMARY KEY (`id`)
);
INSERT INTO animals (id, created_at, enterprise_id, kind_id)
values (1, '2021-01-25 04:36:56', 1, 1),
    (2, '2021-03-09 01:54:46', 1, 1),
    (7, '2021-03-09 01:50:18', 1, 1),
    (8, '2021-03-09 01:50:52', 2, 2),
    (14, '2021-03-09 02:23:51', 2, 2),
    (16, '2021-03-09 01:49:17', 3, 2),
    (17, '2021-03-09 02:31:06', 3, 2),
    (18, '2021-03-09 02:11:46', 3, 3),
    (21, '2021-03-09 02:32:37', 4, 3),
    (22, '2021-03-09 01:49:50', 4, 3),
    (23, '2021-03-09 02:23:16', 4, 4),
    (24, '2021-03-09 02:13:14', 4, 4),
    (26, '2021-03-09 01:48:38', 5, 4),
    (27, '2021-03-09 02:22:49', 5, 5),
    (28, '2021-03-09 01:55:36', 5, 5);
INSERT INTO enterprises (id, created_at, uuid)
values (
        1,
        '2021-01-25 04:36:08',
        '9907dc1a-7d2c-432f-ac5c-5a2943ac9d71'
    ),
    (
        2,
        '2023-04-06 03:31:00',
        '9907dc1a-da82-4034-a976-6c27916e9e34'
    ),
    (
        3,
        '2023-04-06 04:38:41',
        '9907dc1a-e3f9-4e50-9bd8-5591c7f4085b'
    ),
    (
        4,
        '2020-11-23 03:17:22',
        '9907dc1a-ffc2-49e2-a691-09f565ca169b'
    ),
    (
        5,
        '2020-11-25 03:47:08',
        '9907dc1b-131f-4b5d-aee9-0286d6cd3d78'
    );
INSERT INTO enterprise_addresses (id, created_at, enterprise_id, district_code)
values (
        1,
        '2021-01-25 04:36:08',
        1,
        '3b67dc8e-79b1-43f4-8f9e-2b4990a1a922'
    ),
    (
        2,
        '2020-11-23 03:17:22',
        2,
        '25108675-9fd9-4325-a1fd-c392c3feedac'
    ),
    (
        3,
        '2020-11-23 03:17:22',
        3,
        'c4339d8a-d4ef-42e0-be75-6bc551956e5c'
    ),
    (
        4,
        '2020-11-23 03:17:22',
        4,
        'bb9c86eb-30de-4b8f-9ea8-9edc71fc0488'
    ),
    (
        5,
        '2020-11-23 03:17:22',
        5,
        'f7521d33-7cf3-4f6e-bb66-d9e04912b6fc'
    );