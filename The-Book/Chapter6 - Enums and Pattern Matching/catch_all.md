If we want to define a default behavior (like in switch case) and only specify behavior for a few conditions, we can use `other` in match.

If we do not want to use the value returned by the defualt case, we use `_`.

If we do not want the defualt case to do anything, we simply put `_ => ()`. [code](./projects/catch_all/)

More patterns are covered in Chapter 18.