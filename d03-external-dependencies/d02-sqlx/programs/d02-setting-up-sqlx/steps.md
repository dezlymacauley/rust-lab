check-status

connect-admin

_______________________________________________________________________________

CREATE DATABASE rust_lab;

\c rust_lab;

CREATE SCHEMA sqlx_tutorial;

\dn
_______________________________________________________________________________

To undo:

-- 1. Remove the schema
DROP SCHEMA sqlx_tutorial CASCADE;

-- 2. Disconnect from rust_lab
\c postgres

-- 3. Remove the database
DROP DATABASE rust_lab;
_______________________________________________________________________________
