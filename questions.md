
is there automation or development I could do to help out
what issues do you guys face
So cat has 5,000 apps right, Hey I know cat has been around for 100 years is there room to move or migrate those.

what tools do you guys use?
is there more project work or just troubleshooting for that?
Is there other things I could do outside work hours or picking up responsbility? I understand the day to day \


how does your work experience at growmark how does that or what challenges do you face here?

what would you do during an outage?
how would you get to the root cause?

give your specfic work example

Here’s a cleaner version you can say in the interview:

---

Use this:
intro here's some of the tools I've used. I'm lucas I went to isu here are some of the tools I've used I worked at growmark.
At Growmark, I worked as a software developer on internal web and cloud-based systems. I worked with Vue on the frontend and AWS services on the backend, including Lambda, Step Functions, S3, SQS-style event processing, IAM permissions, and database integrations.

One of the main projects I worked on was a CMS/card payment replatforming effort. The system involved file-processing workflows, AWS Lambdas, Step Functions, S3, database calls, and stored procedures. The pipeline could run for 45 minutes to an hour, so troubleshooting required understanding the full flow rather than just one piece of code.

When something failed, the cause could be a bad deployment, missing permission, malformed file, S3 metadata issue, Step Function failure, database timeout, stored procedure performance problem, or load-related issue. That gave me experience using logs, checking recent changes, tracing execution through cloud services, and thinking about the system end to end.

I also worked on S3 file navigation and frontend routing problems. That involved debugging URL encoding, file paths, special characters, frontend state, and how the router interacted with the user interface. It taught me how small edge cases can create real production issues if the application does not handle them correctly.

I also had exposure to deployment configuration, IAM roles and policies, SQL Server/MSSQL stored procedures, file validation, queue-based processing, and making backend workflows more robust when unexpected files or inputs came through. We had one specific issue that was hard to solve which was matching s3 file events when they came into our bucket and then having the events being ingested into our pipeline, where we learned about exactly once processing, there we're multiple file events coming in at the same time and we needed to dedup events which lead to more idempotency where we needed to ensure that one file could only be ingested at a time.


I also worked with older technology and ported an older .net application to a newer server that would fit within our pipeline through an integration this allowed us to keep using the older .net app to avoid more expensive time on development.
We also worked with the accoutning department, different senior engineers and project managers to succesfully make decisions that we're best for our vendors and would keep the system working smoothly.
Overall, Growmark helped me build a troubleshooting mindset. I learned how to follow evidence through logs, deployments, permissions, databases, cloud services, and frontend behavior. That experience translates well to this role because enterprise support is about understanding the environment, isolating the failure, communicating clearly, and helping restore service while also looking for ways to prevent the issue from happening agai

Here’s a polished version for **outage response** and **root cause analysis**:

---

## Outage response answer

If there was an outage, my first priority would be understanding the **impact and scope**. I would want to know what service is affected, who is impacted, when it started, and whether the issue is still actively happening.

From there, I would check whether anything recently changed. That could be a code deployment, infrastructure change, configuration update, database change, permission change, or scheduled job. In my experience at Growmark, a lot of issues came from small changes in one part of the system affecting another part of the workflow.

Then I would start narrowing it down by layer. I would check the deployment pipeline, application logs, CloudWatch logs, Step Function execution history, database health, S3 file inputs, IAM permissions, and any downstream services the application depends on.

For example, if a Lambda-based workflow failed, I would check whether the Lambda actually ran, whether it had the right permissions, whether the input file was valid, whether the database call succeeded, and whether the Step Function failed at a specific state.

Once the immediate issue was identified, I would focus on restoring service. Depending on the situation, that could mean rolling back a deployment, fixing a permission issue, retrying a failed workflow, restarting a service, scaling resources, or escalating to the right team with clear evidence.

After the outage is resolved, I would document what happened, what the root cause was, what fixed it, and what could prevent it from happening again.

---

## Root cause answer

For root cause analysis, I try not to guess too early. I start by building a timeline: when did the issue start, what changed around that time, what systems were involved, and what symptoms were reported.

Then I work backward from the failure. I check logs, deployment history, metrics, permissions, database behavior, inputs, and downstream dependencies. The goal is to figure out where the system first started behaving incorrectly, not just where the error finally showed up.

At Growmark, that was important because many of our systems had multiple moving parts. A failure could look like a Lambda issue, but the real cause might be an IAM permission problem, a malformed S3 file, a failed stored procedure, a database timeout, or an issue in the Step Function orchestration.

One example was the CMS/card payment replatforming project. The workflow involved S3 file processing, AWS Lambdas, Step Functions, database calls, and stored procedures. Since the pipeline could run for 45 minutes to an hour, an error could happen at many different stages.

To find root cause, we would trace the workflow from the file input through each step of the process. We would check whether the file was received correctly, whether the Lambda processed it, whether the Step Function reached the expected state, whether the database procedure completed, and whether the output matched what we expected.

That experience taught me that root cause analysis is about following evidence across the whole system. You have to separate symptoms from causes, check recent changes, validate assumptions, and communicate clearly while you narrow the issue down.
n
