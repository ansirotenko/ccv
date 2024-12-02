# App description 
Application has empty static ui with only one input field.
There are 2 background threads. First reports window.is_focused every 100ms. Second thread makes window visible and focused.

# Problem
Focus will never be restored after simple manipulations.

# To reproduce

1. Run application
2. In the opened window click on input and enter some text
3. Click outside window
As a result background thread will never restore window's focus.

# Why it may be important
Imagine you create small application that must opened by pressing shortcut. That application must be focued, for example to search some text or etc.