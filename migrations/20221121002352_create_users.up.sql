CREATE TABLE "user" (
  id SERIAL NOT NULL PRIMARY KEY,
  username VARCHAR(255) NOT NULL UNIQUE,
  email VARCHAR(255) UNIQUE,
  password TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);

CREATE TRIGGER set_updated_at
BEFORE UPDATE ON "user"
FOR EACH ROW
EXECUTE FUNCTION trigger_set_updated_at();

CREATE TRIGGER encrypt_password_on_insert
BEFORE INSERT ON "user"
FOR EACH ROW
WHEN (NEW.password IS NOT NULL)
EXECUTE FUNCTION trigger_encrypt_password();

CREATE TRIGGER encrypt_password
BEFORE UPDATE ON "user"
FOR EACH ROW
WHEN (OLD.password IS DISTINCT FROM NEW.password)
EXECUTE FUNCTION trigger_encrypt_password();
