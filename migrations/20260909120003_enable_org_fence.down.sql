-- Down: remove the org fence for sapiens module

-- Reverse the org fence for sapiens.analytics_metrics
DROP TRIGGER IF EXISTS analytics_metrics_org_unit_kind_guard ON sapiens.analytics_metrics;
DROP FUNCTION IF EXISTS sapiens.analytics_metrics_org_unit_kind_guard();

DROP POLICY IF EXISTS analytics_metrics_org_unit_isolation ON sapiens.analytics_metrics;
ALTER TABLE sapiens.analytics_metrics NO FORCE ROW LEVEL SECURITY;
ALTER TABLE sapiens.analytics_metrics DISABLE ROW LEVEL SECURITY;

-- Reverse the org fence for sapiens.organization_permissions
DROP TRIGGER IF EXISTS organization_permissions_org_unit_kind_guard ON sapiens.organization_permissions;
DROP FUNCTION IF EXISTS sapiens.organization_permissions_org_unit_kind_guard();

DROP POLICY IF EXISTS organization_permissions_org_unit_isolation ON sapiens.organization_permissions;
ALTER TABLE sapiens.organization_permissions NO FORCE ROW LEVEL SECURITY;
ALTER TABLE sapiens.organization_permissions DISABLE ROW LEVEL SECURITY;

-- Reverse the org fence for sapiens.organization_roles
DROP TRIGGER IF EXISTS organization_roles_org_unit_kind_guard ON sapiens.organization_roles;
DROP FUNCTION IF EXISTS sapiens.organization_roles_org_unit_kind_guard();

DROP POLICY IF EXISTS organization_roles_org_unit_isolation ON sapiens.organization_roles;
ALTER TABLE sapiens.organization_roles NO FORCE ROW LEVEL SECURITY;
ALTER TABLE sapiens.organization_roles DISABLE ROW LEVEL SECURITY;

-- Reverse the org fence for sapiens.organization_users
DROP TRIGGER IF EXISTS organization_users_org_unit_kind_guard ON sapiens.organization_users;
DROP FUNCTION IF EXISTS sapiens.organization_users_org_unit_kind_guard();

DROP POLICY IF EXISTS organization_users_org_unit_isolation ON sapiens.organization_users;
ALTER TABLE sapiens.organization_users NO FORCE ROW LEVEL SECURITY;
ALTER TABLE sapiens.organization_users DISABLE ROW LEVEL SECURITY;

-- Reverse the org fence for sapiens.password_policies
DROP TRIGGER IF EXISTS password_policies_org_unit_kind_guard ON sapiens.password_policies;
DROP FUNCTION IF EXISTS sapiens.password_policies_org_unit_kind_guard();

DROP POLICY IF EXISTS password_policies_org_unit_isolation ON sapiens.password_policies;
ALTER TABLE sapiens.password_policies NO FORCE ROW LEVEL SECURITY;
ALTER TABLE sapiens.password_policies DISABLE ROW LEVEL SECURITY;

