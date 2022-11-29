CREATE OR REPLACE FUNCTION trigger_encrypt_password()
RETURNS TRIGGER AS $$
BEGIN
  NEW.password = crypt(NEW.password, gen_salt('bf', 8));
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
