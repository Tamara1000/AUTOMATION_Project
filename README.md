# AUTOMATION_Project
This project logs in to the Vimeo website via the API and responds to a specific video automatically. 
There is then another logging in to Vimeo through Katalon and check if the response to that specific video has actually been recorded.
In addition to that there is navigation to another video and then you keep the number of likes and the number of views of that video 
and check if the data obtained ia actually correct.

## Run the project
Running the project is very simple. 
I used significant names for variables so that the code documents itself. 

Beyond that the scripts contain functions of: accessing the url, finding test objects in html tree,
verifing text elements and comparing results.

To write the response to the video I used POST while to get it I used the GET method.
For the number of views and likes, I kept them in variables and checked if they are indeed varieties for the data I received from the tests.
Of course it was necessary to generate a token through the API developer tools on Vimeo to access the url, interact with the video - write a comment and import data.
