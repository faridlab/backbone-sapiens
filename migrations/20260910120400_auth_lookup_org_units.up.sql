-- Auth-context membership lookup that predates the session scope (ADR-0028).
--
-- The org fence on sapiens.organization_users is FORCE RLS keyed on
-- `app.scope_unit_ids` — exactly right for tenant-scoped requests. But the
-- session verbs (login / me / refresh) must answer "which org units does this
-- user belong to?" BEFORE a session exists, with no scope set, from the
-- application role (NOBYPASSRLS). Fenced, that read returns zero rows and
-- every login fails with "User belongs to no organization".
--
-- This single-purpose SECURITY DEFINER function is the deliberate exemption:
-- it takes a user id and returns only that user's active org-unit ids. It
-- cannot be steered anywhere else — no table, filter, or predicate is
-- caller-controlled.
--
-- Deployment caveat: SECURITY DEFINER escapes FORCE RLS only when the role
-- that ran this migration is a superuser or carries BYPASSRLS. Migrating as
-- a non-privileged owner leaves the function fenced and the auth verbs
-- silently resolving zero memberships; such deployments must re-own the
-- function to a privileged role.

CREATE OR REPLACE FUNCTION sapiens.active_org_units(p_user_id uuid)
RETURNS TABLE (org_unit_id uuid)
LANGUAGE sql
STABLE
SECURITY DEFINER
-- A SECURITY DEFINER body resolves unqualified names with the invoker's
-- privileges; pinning the search path closes the hijack window.
SET search_path = sapiens, public
AS $$
    SELECT ou.org_unit_id
    FROM sapiens.organization_users ou
    WHERE ou.user_id = p_user_id
      AND ou.status = 'active'
    ORDER BY ou.joined_at
$$;

-- Executable only by roles the consumer grants explicitly (see each
-- service's rls_app_role bootstrap); never by PUBLIC.
REVOKE ALL ON FUNCTION sapiens.active_org_units(uuid) FROM PUBLIC;
