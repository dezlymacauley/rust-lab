/*
    ABOUT: PostgreSQL database and schema setup

    Database Name: rust_lab
    Database Schemas: public, sqlx_tutorial

    NOTE: The database schema `public` is automatically created when 
    a database is created.
*/

-- SECTION: 1 => Create the database if it doesn't exist

SELECT 'CREATE DATABASE rust_lab'
WHERE NOT EXISTS (
    SELECT
        FROM pg_database
        WHERE datname = 'rust_lab'
)
\gexec
-------------------------------------------------------------------------------

-- SECTION: 2 => Connect to the application database

\connect rust_lab
-------------------------------------------------------------------------------

-- SECTION: 3 => Create the application schema

CREATE SCHEMA IF NOT EXISTS sqlx_tutorial;
-------------------------------------------------------------------------------
