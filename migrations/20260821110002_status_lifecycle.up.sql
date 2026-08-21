-- Migration: replace lifecycle booleans with status enums
--
-- Tree-wide convention: one `status` enum field per lifecycle, no boolean
-- impostors (docs/refactoring-schema in the serpa workspace). Each boolean
-- migrates only the rows deviating from its own column default, so folds
-- never clobber rows already carrying a more specific state. Enum types are
-- created unqualified so they land beside the module's other enum types in
-- public, where the generated sqlx type_name resolves. `notification_template_status`
-- is identical in shape to the type the backbone-notification module creates
-- for its own table; the DO-block guard keeps either module deployable alone.

DO $$ BEGIN CREATE TYPE session_status AS ENUM ('active', 'revoked'); EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE TYPE oauth_provider_status AS ENUM ('active', 'inactive'); EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE TYPE organization_role_status AS ENUM ('active', 'inactive'); EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE TYPE organization_permission_status AS ENUM ('active', 'inactive'); EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE TYPE password_policy_status AS ENUM ('active', 'inactive'); EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE TYPE user_permission_status AS ENUM ('active', 'inactive'); EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE TYPE workflow_definition_status AS ENUM ('active', 'inactive'); EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE TYPE notification_template_status AS ENUM ('active', 'inactive'); EXCEPTION WHEN duplicate_object THEN NULL; END $$;

-- sessions: revocation derives from revoked_at (the every-request read path
-- already keys on revoked_at IS NULL AND expires_at > NOW()); is_active was a
-- stale mirror no revoke path ever set, so it is dropped without translation.
ALTER TABLE sapiens.sessions ADD COLUMN status session_status NOT NULL DEFAULT 'active';
UPDATE sapiens.sessions SET status = 'revoked' WHERE revoked_at IS NOT NULL;
DROP INDEX IF EXISTS idx_sessions_user_id_is_active;
CREATE INDEX idx_sessions_user_id_status ON sapiens.sessions (user_id, status);
ALTER TABLE sapiens.sessions DROP COLUMN is_active;

-- oauth_providers: provider enabled/disabled becomes the status lifecycle.
ALTER TABLE sapiens.oauth_providers ADD COLUMN status oauth_provider_status NOT NULL DEFAULT 'active';
UPDATE sapiens.oauth_providers SET status = 'inactive' WHERE NOT is_active;
DROP INDEX IF EXISTS idx_oauth_providers_is_active;
CREATE INDEX idx_oauth_providers_status ON sapiens.oauth_providers (status);
ALTER TABLE sapiens.oauth_providers DROP COLUMN is_active;

-- organization_roles
ALTER TABLE sapiens.organization_roles ADD COLUMN status organization_role_status NOT NULL DEFAULT 'active';
UPDATE sapiens.organization_roles SET status = 'inactive' WHERE NOT is_active;
DROP INDEX IF EXISTS idx_organization_roles_is_active;
CREATE INDEX idx_organization_roles_status ON sapiens.organization_roles (status);
ALTER TABLE sapiens.organization_roles DROP COLUMN is_active;

-- organization_permissions (carries no is_active index)
ALTER TABLE sapiens.organization_permissions ADD COLUMN status organization_permission_status NOT NULL DEFAULT 'active';
UPDATE sapiens.organization_permissions SET status = 'inactive' WHERE NOT is_active;
ALTER TABLE sapiens.organization_permissions DROP COLUMN is_active;

-- password_policies
ALTER TABLE sapiens.password_policies ADD COLUMN status password_policy_status NOT NULL DEFAULT 'active';
UPDATE sapiens.password_policies SET status = 'inactive' WHERE NOT is_active;
DROP INDEX IF EXISTS idx_password_policies_is_active;
CREATE INDEX idx_password_policies_status ON sapiens.password_policies (status);
ALTER TABLE sapiens.password_policies DROP COLUMN is_active;

-- user_permissions (this table lives in public, not sapiens)
ALTER TABLE public.user_permissions ADD COLUMN status user_permission_status NOT NULL DEFAULT 'active';
UPDATE public.user_permissions SET status = 'inactive' WHERE NOT is_active;
DROP INDEX IF EXISTS idx_user_permissions_user_id_is_active;
CREATE INDEX idx_user_permissions_user_id_status ON public.user_permissions (user_id, status);
ALTER TABLE public.user_permissions DROP COLUMN is_active;

-- workflow_definitions
ALTER TABLE sapiens.workflow_definitions ADD COLUMN status workflow_definition_status NOT NULL DEFAULT 'active';
UPDATE sapiens.workflow_definitions SET status = 'inactive' WHERE NOT is_active;
DROP INDEX IF EXISTS idx_workflow_definitions_is_active;
CREATE INDEX idx_workflow_definitions_status ON sapiens.workflow_definitions (status);
ALTER TABLE sapiens.workflow_definitions DROP COLUMN is_active;

-- notification_templates (type shared with backbone-notification)
ALTER TABLE sapiens.notification_templates ADD COLUMN status notification_template_status NOT NULL DEFAULT 'active';
UPDATE sapiens.notification_templates SET status = 'inactive' WHERE NOT is_active;
DROP INDEX IF EXISTS idx_notification_templates_is_active;
CREATE INDEX idx_notification_templates_status ON sapiens.notification_templates (status);
ALTER TABLE sapiens.notification_templates DROP COLUMN is_active;

-- ldap_directories: fold into the existing status enum; the conditional
-- update protects draft and error rows.
UPDATE sapiens.ldap_directories SET status = 'inactive' WHERE NOT is_active;
DROP INDEX IF EXISTS idx_ldap_directories_is_active;
ALTER TABLE sapiens.ldap_directories DROP COLUMN is_active;

-- saml_providers: fold; the conditional update protects draft rows.
UPDATE sapiens.saml_providers SET status = 'inactive' WHERE NOT is_active;
DROP INDEX IF EXISTS idx_saml_providers_is_active;
ALTER TABLE sapiens.saml_providers DROP COLUMN is_active;

-- mfa_devices: lockout folds into status (variant added by the previous
-- stamp). is_active defaulted false while status defaulted active — every
-- default-enrolled row carried false — so it is dropped WITHOUT translation;
-- any translation would deactivate the whole device fleet.
UPDATE sapiens.mfa_devices SET status = 'locked' WHERE is_locked;
DROP INDEX IF EXISTS idx_mfa_devices_is_locked_status;
CREATE INDEX idx_mfa_devices_status ON sapiens.mfa_devices (status);
ALTER TABLE sapiens.mfa_devices DROP COLUMN is_locked;
ALTER TABLE sapiens.mfa_devices DROP COLUMN is_active;

-- user_oauth_links: fold into link_status (renamed to status); a deactivated
-- link is `revoked`; the conditional update protects pending links.
UPDATE sapiens.user_oauth_links SET link_status = 'revoked' WHERE NOT is_active;
ALTER TABLE sapiens.user_oauth_links RENAME COLUMN link_status TO status;
ALTER TABLE sapiens.user_oauth_links DROP COLUMN is_active;
