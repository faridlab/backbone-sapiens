-- The auth membership lookup counts only memberships that are still current.
--
-- sapiens.active_org_units answers "which org units may this user sign in to?" for login,
-- refresh and acting-unit checks. A membership that has ended (left_at set) or been deleted
-- (soft delete) kept answering, so someone who left a unit while keeping another membership
-- went on minting tokens for the unit they left. The body is replaced in place; its grants and
-- SECURITY DEFINER exemption are unchanged (see 20260910120400).

CREATE OR REPLACE FUNCTION sapiens.active_org_units(p_user_id uuid)
RETURNS TABLE (org_unit_id uuid)
LANGUAGE sql
STABLE
SECURITY DEFINER
SET search_path = sapiens, public
AS $$
    SELECT ou.org_unit_id
    FROM sapiens.organization_users ou
    WHERE ou.user_id = p_user_id
      AND ou.status = 'active'
      AND ou.left_at IS NULL
      AND (ou.metadata->>'deleted_at') IS NULL
    ORDER BY ou.joined_at
$$;
