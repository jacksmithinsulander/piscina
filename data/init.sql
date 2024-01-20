-- File: create_table.sql

CREATE TABLE IF NOT EXISTS `found_pools` (
    `uid` INT AUTO_INCREMENT PRIMARY KEY,
    `chain` VARCHAR(255) NOT NULL,
    `time_of_creation` INT,
    `token_a_name` VARCHAR(255) NOT NULL,
    `token_a_symbol` VARCHAR(255) NOT NULL,
    `token_a_amount` INT,
    `token_a_price` INT, 
    `token_b_name` VARCHAR(255) NOT NULL,
    `token_b_symbol` VARCHAR(255) NOT NULL,
    `token_b_amount` INT,
    `token_b_price` INT,
);