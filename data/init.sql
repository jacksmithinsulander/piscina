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
    `token_b_price` INT
);

INSERT INTO found_pools (
    chain,
    time_of_creation,
    token_a_name,
    token_a_symbol,
    token_a_amount,
    token_a_price,
    token_b_name,
    token_b_symbol,
    token_b_amount,
    token_b_price
) VALUES (
    'bsc',
    1706647507,
    'ETHEREUM',
    'ETH',
    30,  -- Replace with the actual token A amount value
    1000000,  -- Replace with the actual token A price value
    'AMPLEFORTH',
    'AMPL',
    1000000,  -- Replace with the actual token B amount value
    0.0003   -- Replace with the actual token B price value
);
