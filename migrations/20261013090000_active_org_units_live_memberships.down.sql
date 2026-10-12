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
    ORDER BY ou.joined_at
$$;
