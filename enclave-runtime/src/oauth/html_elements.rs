pub static HTML_404: &str ="
<!DOCTYPE html>
<html lang='en'>
  <head>
    <meta charset='utf-8'>
    <title>Hello!</title>
  </head>
  <body>
    <h1>Oops!</h1>
    <p>Sorry, I don't know what you're asking for.</p>
  </body>
</html>
";

pub static HTML_RESOURCE_CONTENT: &str = "
<!DOCTYPE html>
<html lang='en'>
  <head>
      <title>Resource Content</title>
  </head>
  <body>
      <h1>Welcome to the Resource!</h1>
      <p>This is the content of the protected resource.</p>
  </body>
</html>
";

pub static HTML_ACCESS_DENIED: &str = "
<!DOCTYPE html>
<html lang='en'>
    Access Denied
</html>
";