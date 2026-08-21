-- Down for status_lifecycle: restore the lifecycle booleans.
--
-- Best-effort restore: the fold translations are lossy in reverse — rows that
-- entered `inactive` / `locked` / `revoked` after the up stamp map back to
-- false, and pre-existing folded rows cannot be distinguished from them. The
-- `locked` variant added by 20260821110001 lingers in mfa_device_status
-- (enum values cannot be dropped in place).

ALTER TABLE sapiens.user_oauth_links ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT true;
UPDATE sapiens.user_oauth_links SET is_active = false WHERE status = 'revoked';
ALTER TABLE sapiens.user_oauth_links RENAME COLUMN status TO link_status;

ALTER TABLE sapiens.mfa_devices ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE sapiens.mfa_devices ADD COLUMN is_locked BOOLEAN NOT NULL DEFAULT false;
UPDATE sapiens.mfa_devices SET is_locked = true WHERE status = 'locked';
DROP INDEX IF EXISTS idx_mfa_devices_status;

ALTER TABLE sapiens.saml_providers ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT true;
UPDATE sapiens.saml_providers SET is_active = false WHERE status = 'inactive';

ALTER TABLE sapiens.ldap_directories ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT true;
UPDATE sapiens.ldap_directories SET is_active = false WHERE status = 'inactive';

ALTER TABLE sapiens.notification_templates ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT true;
UPDATE sapiens.notification_templates SET is_active = false WHERE status = 'inactive';
DROP INDEX IF EXISTS idx_notification_templates_status;
CREATE INDEX idx_notification_templates_is_active ON sapiens.notification_templates (is_active);
ALTER TABLE sapiens.notification_templates DROP COLUMN status;

ALTER TABLE sapiens.workflow_definitions ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT true;
UPDATE sapiens.workflow_definitions SET is_active = false WHERE status = 'inactive';
DROP INDEX IF EXISTS idx_workflow_definitions_status;
CREATE INDEX idx_workflow_definitions_is_active ON sapiens.workflow_definitions (is_active);
ALTER TABLE sapiens.workflow_definitions DROP COLUMN status;

ALTER TABLE public.user_permissions ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT true;
UPDATE public.user_permissions SET is_active = false WHERE status = 'inactive';
DROP INDEX IF EXISTS idx_user_permissions_user_id_status;
CREATE INDEX idx_user_permissions_user_id_is_active ON public.user_permissions (user_id, is_active);
ALTER TABLE public.user_permissions DROP COLUMN status;

ALTER TABLE sapiens.password_policies ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT true;
UPDATE sapiens.password_policies SET is_active = false WHERE status = 'inactive';
DROP INDEX IF EXISTS idx_password_policies_status;
CREATE INDEX idx_password_policies_is_active ON sapiens.password_policies (is_active);
ALTER TABLE sapiens.password_policies DROP COLUMN status;

ALTER TABLE sapiens.organization_permissions ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT true;
UPDATE sapiens.organization_permissions SET is_active = false WHERE status = 'inactive';
ALTER TABLE sapiens.organization_permissions DROP COLUMN status;

ALTER TABLE sapiens.organization_roles ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT true;
UPDATE sapiens.organization_roles SET is_active = false WHERE status = 'inactive';
DROP INDEX IF EXISTS idx_organization_roles_status;
CREATE INDEX idx_organization_roles_is_active ON sapiens.organization_roles (is_active);
ALTER TABLE sapiens.organization_roles DROP COLUMN status;

ALTER TABLE sapiens.oauth_providers ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT true;
UPDATE sapiens.oauth_providers SET is_active = false WHERE status = 'inactive';
DROP INDEX IF EXISTS idx_oauth_providers_status;
CREATE INDEX idx_oauth_providers_is_active ON sapiens.oauth_providers (is_active);
ALTER TABLE sapiens.oauth_providers DROP COLUMN status;

ALTER TABLE sapiens.sessions ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT true;
UPDATE sapiens.sessions SET is_active = false WHERE status = 'revoked';
DROP INDEX IF EXISTS idx_sessions_user_id_status;
CREATE INDEX idx_sessions_user_id_is_active ON sapiens.sessions (user_id, is_active);
ALTER TABLE sapiens.sessions DROP COLUMN status;

-- Type drops are last, guarded: notification_template_status is shared with
-- the backbone-notification module and only drops when this module was its
-- last user.
DROP TYPE IF EXISTS session_status;
DROP TYPE IF EXISTS oauth_provider_status;
DROP TYPE IF EXISTS organization_role_status;
DROP TYPE IF EXISTS organization_permission_status;
DROP TYPE IF EXISTS password_policy_status;
DROP TYPE IF EXISTS user_permission_status;
DROP TYPE IF EXISTS workflow_definition_status;
DROP TYPE IF EXISTS notification_template_status;
