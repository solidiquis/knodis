CREATE OR REPLACE FUNCTION authenticate_via_pw(uname VARCHAR(255), pw VARCHAR(72))
RETURNS RECORD AS $$
DECLARE
  authenticated_user RECORD;
BEGIN
  SELECT
    "user".*
  FROM
    "user"
  WHERE
    username = uname
    AND crypt(pw, password)
  INTO
    authenticated_user;

  RETURN authenticated_user;
END;
$$ LANGUAGE plpgsql;
