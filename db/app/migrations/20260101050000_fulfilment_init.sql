-- migrate:up

-- The schema and role of nanoservice `fulfilment`. Every table it owns
-- lives in nano_fulfilment and belongs to the nano_fulfilment role;
-- only that role may touch them (the Directive §2, made mechanical). The
-- `app` login is a member of every nanoservice role WITH SET so each pool
-- can SET ROLE to its own; it owns the schema (it is the migrator, and the
-- owner of the envelope parent, so it is the one that creates a type's
-- partition in here) but nothing IN it, so an unswitched query against a
-- nanoservice's table is a permission error and only a deliberate SET ROLE
-- escapes.

-- The role is cluster-global and two databases may run this migration at
-- once (a test database per test). The existence check is not atomic, so
-- the concurrent loser's CREATE ROLE is caught and the check repeated: the
-- repeat is what makes the winner's committed role visible to the rest of
-- this transaction (its scan of pg_roles accepts the catalog invalidation
-- the winner's commit sent, which the failed CREATE ROLE left a stale
-- negative cache entry behind for).
DO $$
DECLARE
    attempts int := 0;
BEGIN
    LOOP
        IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'nano_fulfilment') THEN
            EXIT;
        END IF;
        BEGIN
            CREATE ROLE nano_fulfilment NOLOGIN NOINHERIT;
            EXIT;
        EXCEPTION WHEN duplicate_object OR unique_violation THEN
            attempts := attempts + 1;
            IF attempts > 50 THEN
                RAISE;
            END IF;
            PERFORM pg_sleep(0.05);
        END;
    END LOOP;
END $$;

GRANT nano_fulfilment TO app WITH SET TRUE, INHERIT FALSE;

CREATE SCHEMA IF NOT EXISTS nano_fulfilment;
GRANT USAGE, CREATE ON SCHEMA nano_fulfilment TO nano_fulfilment;
ALTER ROLE nano_fulfilment SET search_path = nano_fulfilment;

GRANT USAGE ON SCHEMA basable TO nano_fulfilment;
GRANT SELECT ON basable.processing_object_type TO nano_fulfilment;
GRANT USAGE ON SCHEMA basable_config TO nano_fulfilment;
GRANT SELECT ON ALL TABLES IN SCHEMA basable_config TO nano_fulfilment;
ALTER DEFAULT PRIVILEGES IN SCHEMA basable_config GRANT SELECT ON TABLES TO nano_fulfilment;

-- migrate:down
DROP SCHEMA IF EXISTS nano_fulfilment CASCADE;
REVOKE nano_fulfilment FROM app;
DROP ROLE IF EXISTS nano_fulfilment;
