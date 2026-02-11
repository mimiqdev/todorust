# Todoist v1 Sync API Documentation

**官方文档链接**: https://developer.todoist.com/api/v1  
**获取日期**: 2026-02-11  
**API 版本**: v1 (当前版本)  
**说明**: Todoist v1 API 包含了原来 Sync API v9 和 REST API v2 的功能

---

## 目录

1. [概述](#概述)
2. [读取资源](#读取资源)
3. [写入资源](#写入资源)
4. [命令 UUID](#命令-uuid)
5. [临时资源 ID](#临时资源-id)
6. [响应和错误](#响应和错误)
7. [批量命令](#批量命令)
8. [增量同步](#增量同步)
9. [资源类型列表](#资源类型列表)
10. [Sections 相关信息](#sections-相关信息)

---

<p>The Todoist Sync endpoint is specially designed for efficient data sync between
clients (e.g. our mobile apps) and Todoist.</p>
<p>Sync requests should be made in HTTP POST (<code>application/x-www-form-urlencoded</code>).
Sync responses, including errors, will be returned in JSON.</p>
<p>The Sync endpoint supports the following features:</p>
<ul>
<li><a href="#tag/Sync/Overview/Batching-commands">Batching</a>: reading and writing of
multiple resources can be done in a single HTTP request. Batch requests help
clients reduce the number of network calls needed to sync resources.</li>
<li><a href="#tag/Sync/Overview/Incremental-sync">Incremental sync</a>: You only retrieve
data that is updated since the last time you performed a sync request.</li>
</ul>
<p><em>Refer to <a href="#tag/Request-limits">Request Limits</a> to learn more about the number of requests/commands
you have for the Sync API</em></p>

<blockquote>
<p>Example read resources request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">sync_token</span><span class="token operator">=</span><span class="token string">'*'</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">resource_types</span><span class="token operator">=</span><span class="token string">'["all"]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token string">"completed_info"</span><span class="token builtin class-name">:</span> <span class="token punctuation">[</span> <span class="token punctuation">..</span>. <span class="token punctuation">]</span>,
  <span class="token string">"collaborators"</span><span class="token builtin class-name">:</span> <span class="token punctuation">[</span> <span class="token punctuation">..</span>. <span class="token punctuation">]</span>,
  <span class="token string">"collaborator_states"</span><span class="token builtin class-name">:</span> <span class="token punctuation">[</span> <span class="token punctuation">..</span>. <span class="token punctuation">]</span>,
  <span class="token string">"day_orders"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span> <span class="token punctuation">..</span>. <span class="token punctuation">}</span>,
  <span class="token string">"filters"</span><span class="token builtin class-name">:</span> <span class="token punctuation">[</span> <span class="token punctuation">..</span>. <span class="token punctuation">]</span>,
  <span class="token string">"full_sync"</span><span class="token builtin class-name">:</span> true,
  <span class="token string">"items"</span><span class="token builtin class-name">:</span> <span class="token punctuation">[</span> <span class="token punctuation">..</span>. <span class="token punctuation">]</span>,
  <span class="token string">"labels"</span><span class="token builtin class-name">:</span> <span class="token punctuation">[</span> <span class="token punctuation">..</span>. <span class="token punctuation">]</span>,
  <span class="token string">"live_notifications"</span><span class="token builtin class-name">:</span> <span class="token punctuation">[</span> <span class="token punctuation">..</span>. <span class="token punctuation">]</span>,
  <span class="token string">"live_notifications_last_read_id"</span><span class="token builtin class-name">:</span> <span class="token string">"0"</span>,
  <span class="token string">"locations"</span><span class="token builtin class-name">:</span> <span class="token punctuation">[</span> <span class="token punctuation">..</span>. <span class="token punctuation">]</span>,
  <span class="token string">"notes"</span><span class="token builtin class-name">:</span> <span class="token punctuation">[</span> <span class="token punctuation">..</span>. <span class="token punctuation">]</span>,
  <span class="token string">"project_notes"</span><span class="token builtin class-name">:</span> <span class="token punctuation">[</span> <span class="token punctuation">..</span>. <span class="token punctuation">]</span>,
  <span class="token string">"projects"</span><span class="token builtin class-name">:</span> <span class="token punctuation">[</span> <span class="token punctuation">..</span>. <span class="token punctuation">]</span>,
  <span class="token string">"project_view_options_defaults"</span><span class="token builtin class-name">:</span> <span class="token punctuation">[</span> <span class="token punctuation">..</span>. <span class="token punctuation">]</span>,
  <span class="token string">"reminders"</span><span class="token builtin class-name">:</span> <span class="token punctuation">[</span> <span class="token punctuation">..</span>. <span class="token punctuation">]</span>,
  <span class="token string">"role_actions"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span> <span class="token punctuation">..</span>. <span class="token punctuation">}</span>,
  <span class="token string">"sections"</span><span class="token builtin class-name">:</span> <span class="token punctuation">[</span> <span class="token punctuation">..</span>. <span class="token punctuation">]</span>,
  <span class="token string">"stats"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span> <span class="token punctuation">..</span>. <span class="token punctuation">}</span>,
  <span class="token string">"settings_notifications"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span> <span class="token punctuation">..</span>. <span class="token punctuation">}</span>,
  <span class="token string">"sync_token"</span><span class="token builtin class-name">:</span> <span class="token string">"TnYUZEpuzf2FMA9qzyY3j4xky6dXiYejmSO85S5paZ_a9y1FI85mBbIWZGpW"</span>,
  <span class="token string">"temp_id_mapping"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span> <span class="token punctuation">..</span>. <span class="token punctuation">}</span>,
  <span class="token string">"user"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span> <span class="token punctuation">..</span>. <span class="token punctuation">}</span>,
  <span class="token string">"user_plan_limits"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span> <span class="token punctuation">..</span>. <span class="token punctuation">}</span>,
  <span class="token string">"user_settings"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span> <span class="token punctuation">..</span>. <span class="token punctuation">}</span>,
  <span class="token string">"view_options"</span><span class="token builtin class-name">:</span> <span class="token punctuation">[</span> <span class="token punctuation">..</span>. <span class="token punctuation">]</span>,
  <span class="token string">"workspace_users"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span> <span class="token punctuation">..</span>. <span class="token punctuation">}</span>
<span class="token punctuation">}</span>
</code></pre>
<p>To retrieve your user resources, make a Sync API request with the following
parameters:</p>
<h4 id="parameters">Parameters</h4>
<table>
<thead>
<tr>
<th>Parameter</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>sync_token <em>String</em></td>
<td>Yes</td>
<td>A special string, used to allow the client to perform incremental sync. Pass <code>*</code> to retrieve all active resource data. More details about this below.</td>
</tr>
<tr>
<td>resource_types <em>JSON array of strings</em></td>
<td>Yes</td>
<td>Used to specify what resources to fetch from the server. It should be a JSON-encoded array of strings. Here is a list of available resource types: <code>labels</code>, <code>projects</code>, <code>items</code>, <code>notes</code>, <code>sections</code>, <code>filters</code>, <code>reminders</code>, <code>reminders_location</code>, <code>locations</code>, <code>user</code>, <code>live_notifications</code>, <code>collaborators</code>, <code>user_settings</code>, <code>notification_settings</code>, <code>user_plan_limits</code>, <code>completed_info</code>, <code>stats</code>, <code>workspaces</code>, <code>workspace_users</code>, <code>workspace_filters</code>, <code>view_options</code>, <code>project_view_options_defaults</code>, <code>role_actions</code>. You may use <code>all</code> to include all the resource types. Resources can also be excluded by prefixing a <code>-</code> prior to the name, for example, <code>-projects</code></td>
</tr>
</tbody></table>
<p>In order to fetch both types of reminders you must include both resource types in your request, for example: <code>resource_types=[&quot;reminders&quot;, &quot;reminders_location&quot;]</code> .</p>
<p>The <code>workspace_users</code> resource type will not be returned in full sync, but should be requested in incremental sync to keep data up-to-date once it&#39;s loaded from the REST endpoint.</p>
<h4 id="response">Response</h4>
<p>When the request succeeds, an HTTP 200 response will be returned with a JSON
object containing the requested resources and a new <code>sync_token</code>.</p>
<table>
<thead>
<tr>
<th>Field</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>sync_token</td>
<td>A new synchronization token. Used by the client in the next sync request to perform an incremental sync.</td>
</tr>
<tr>
<td>full_sync</td>
<td>Whether the response contains all data (a full synchronization) or just the incremental updates since the last sync.</td>
</tr>
<tr>
<td>full_sync_date_utc</td>
<td>For full syncs, the time when the data was generated. For big accounts, the data may be returned with some delay, requiring an <a href="#tag/Sync/Overview/Incremental-sync">incremental sync</a> to get up-to-date data.</td>
</tr>
<tr>
<td>user</td>
<td>A user object.</td>
</tr>
<tr>
<td>projects</td>
<td>An array of <a href="#tag/Sync/Projects">project</a> objects.</td>
</tr>
<tr>
<td>items</td>
<td>An array of <a href="#tag/Sync/Items">item</a> objects.</td>
</tr>
<tr>
<td>notes</td>
<td>An array of <a href="#tag/Sync/Comments/Task-Comments">task comments</a> objects.</td>
</tr>
<tr>
<td>project_notes</td>
<td>An array of <a href="#tag/Sync/Comments/Project-Comments">project comments</a> objects.</td>
</tr>
<tr>
<td>sections</td>
<td>An array of <a href="#tag/Sync/Sections">section</a> objects.</td>
</tr>
<tr>
<td>labels</td>
<td>An array of <a href="#tag/Sync/Labels">personal label</a> objects.</td>
</tr>
<tr>
<td>filters</td>
<td>An array of <a href="#tag/Sync/Filters">filter</a> objects.</td>
</tr>
<tr>
<td>workspace_filters</td>
<td>An array of <a href="#tag/Sync/Workspace-Filters">workspace filter</a> objects.</td>
</tr>
<tr>
<td>day_orders</td>
<td>A JSON object specifying the order of items in daily agenda.</td>
</tr>
<tr>
<td>reminders</td>
<td>An array of <a href="#tag/Sync/Reminders">reminder</a> objects.</td>
</tr>
<tr>
<td>collaborators</td>
<td>A JSON object containing all <a href="#tag/Sync/Sharing/Collaborators">collaborators</a> for all shared projects. The <code>projects</code> field contains the list of all shared projects, where the user acts as one of collaborators.</td>
</tr>
<tr>
<td>collaborators_states</td>
<td>An array specifying the state of each collaborator in each project. The state can be invited, active, inactive, deleted.</td>
</tr>
<tr>
<td>completed_info</td>
<td>An array of <code>completed</code> info objects indicating the number of completed items within an active project, section, or parent item. Projects will also include the number of archived sections.</td>
</tr>
<tr>
<td>live_notifications</td>
<td>An array of <code>live_notification</code> objects.</td>
</tr>
<tr>
<td>live_notifications_last_read</td>
<td>What is the last live notification the user has seen? This is used to implement unread notifications.</td>
</tr>
<tr>
<td>user_settings</td>
<td>A JSON object containing <a href="#tag/Sync/User/User-settings">user settings</a>.</td>
</tr>
<tr>
<td>user_plan_limits</td>
<td>A JSON object containing <a href="#tag/Sync/User/User-plan-limits">user plan limits</a>.</td>
</tr>
<tr>
<td>stats</td>
<td>A JSON object containing <a href="#tag/Sync/User/User-productivity-stats">user productivity stats</a> with completion counts for today and this week.</td>
</tr>
<tr>
<td>view_options</td>
<td>An array of <a href="#tag/Sync/View-options">view options</a> objects.</td>
</tr>
<tr>
<td>project_view_options_defaults</td>
<td>An array of <a href="#tag/Sync/Project-View-Options-Defaults">project view options defaults</a> objects.</td>
</tr>
<tr>
<td>role_actions</td>
<td>The actions each role in the system are allowed to perform on a project</td>
</tr>
<tr>
<td>workspaces</td>
<td>A JSON object containing <a href="#tag/Sync/Workspace">workspace</a> objects.</td>
</tr>
<tr>
<td>workspace_users</td>
<td>A JSON object containing <a href="#tag/Sync/Workspace-users">workspace_user</a> objects. Only in incremental sync.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example create project request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "project_add",
        "temp_id": "381e601f-0ef3-4ed6-bf95-58f896d1a314",
        "uuid": "ed1ce597-e4c7-4a88-ba48-e048d827c067",
        "args": {
            "name": "Shopping List",
            "color": "berry_red"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token string">"sync_token"</span><span class="token builtin class-name">:</span> <span class="token string">"cdTUEvJoChiaMysD7vJ14UnhN-FRdP-IS3aisOUpl3aGlIQA9qosBgvMmhbn"</span>,
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"ed1ce597-e4c7-4a88-ba48-e048d827c067"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token string">"temp_id_mapping"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"381e601f-0ef3-4ed6-bf95-58f896d1a314"</span><span class="token builtin class-name">:</span> <span class="token string">"6HWcc9PJCvPjCxC9"</span><span class="token punctuation">}</span>
<span class="token punctuation">}</span>
</code></pre>
<p>To write to your user&#39;s Todoist resources, make a Sync API request with the
following parameters:</p>
<h4 id="parameters">Parameters</h4>
<table>
<thead>
<tr>
<th>Parameter</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>commands <em>JSON</em></td>
<td>Yes</td>
<td>A JSON array of Command objects. Each command will be processed in the specified order.</td>
</tr>
</tbody></table>
<h4 id="command-object">Command object</h4>
<table>
<thead>
<tr>
<th>Field</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>type <em>String</em></td>
<td>The type of the command.</td>
</tr>
<tr>
<td>args <em>Object</em></td>
<td>The parameters of the command.</td>
</tr>
<tr>
<td>uuid <em>String</em></td>
<td>Command UUID. More details about this below.</td>
</tr>
<tr>
<td>temp_id <em>String</em></td>
<td>Temporary resource ID, Optional. Only specified for commands that create a new resource (e.g. <code>item_add</code> command). More details about this below.</td>
</tr>
</tbody></table>

<p>Clients should generate a unique string ID for each command and specify it
in the <code>uuid</code> field. The Command UUID will be used for two purposes:</p>
<ol>
<li>Command result mapping: Each command&#39;s result will be stored in the
<code>sync_status</code> field of the response JSON object. The <code>sync_status</code> object has
its key mapped to a command&#39;s <code>uuid</code> and its value containing the result of a
command.</li>
<li>Command idempotency: Todoist will not execute a command that has same UUID as
a previously executed command. This will allow clients to safely retry
each command without accidentally performing the action twice.</li>
</ol>

<blockquote>
<p>An example that shows how temporary IDs can be used and referenced:</p>
</blockquote>
<pre><code><span class="token punctuation">[</span>
    <span class="token punctuation">{</span>
        <span class="token string">"type"</span><span class="token punctuation">:</span> <span class="token string">"project_add"</span><span class="token punctuation">,</span>
        <span class="token string">"temp_id"</span><span class="token punctuation">:</span> <span class="token string">"c7beb07f-b226-4eb1-bf63-30d782b07b1a"</span><span class="token punctuation">,</span>
        <span class="token string">"args"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span>
            <span class="token string">"name"</span><span class="token punctuation">:</span> <span class="token string">"Shopping List"</span>
        <span class="token punctuation">}</span><span class="token punctuation">,</span>
        <span class="token string">"uuid"</span><span class="token punctuation">:</span> <span class="token string">"ac417051-1cdc-4dc3-b4f8-14526d5bfd16"</span>
    <span class="token punctuation">}</span><span class="token punctuation">,</span>
    <span class="token punctuation">{</span>
        <span class="token string">"type"</span><span class="token punctuation">:</span> <span class="token string">"item_add"</span><span class="token punctuation">,</span>
        <span class="token string">"temp_id"</span><span class="token punctuation">:</span> <span class="token string">"43f7ed23-a038-46b5-b2c9-4abda9097ffa"</span><span class="token punctuation">,</span>
        <span class="token string">"args"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span>
            <span class="token string">"content"</span><span class="token punctuation">:</span> <span class="token string">"Buy Milk"</span><span class="token punctuation">,</span>
            <span class="token string">"project_id"</span><span class="token punctuation">:</span> <span class="token string">"c7beb07f-b226-4eb1-bf63-30d782b07b1a"</span>
        <span class="token punctuation">}</span><span class="token punctuation">,</span>
        <span class="token string">"uuid"</span><span class="token punctuation">:</span> <span class="token string">"849fff4e-8551-4abb-bd2a-838d092775d7"</span>
    <span class="token punctuation">}</span>
<span class="token punctuation">]</span>
</code></pre>
<blockquote>
<p>You can see that the <code>project_add</code> command specified a <code>temp_id</code> property
(<code>c7beb07f-b226-4eb1-bf63-30d782b07b1a</code>) as placeholder of the actual
<code>project_id</code>. The <code>item_add</code> command can reference to this temporary project
ID. The API will automatically resolve these IDs.</p>
</blockquote>
<p>Some commands depend on the result of previous command. For instance, you have a
command sequence: <code>&quot;project_add&quot;</code> and <code>&quot;item_add&quot;</code> which first creates a project
and then add a new task to the newly created project. In order to run the later
<code>item_add</code> command, we need to obtain the project ID returned from the previous
command. Therefore, the normal approach would be to run these two commands in
two separate HTTP requests.</p>
<p>The temporary resource ID feature allows you to run two or more dependent
commands in a single HTTP request. For commands that are related to creation of
resources (i.e. <code>item_add</code>, <code>project_add</code>), you can specify an extra <code>temp_id</code>
as a placeholder for the actual ID of the resource. The other commands in the
same sequence could directly refer to <code>temp_id</code> if needed.</p>

<blockquote>
<p>An example of a single request sync return value:</p>
</blockquote>
<pre><code><span class="token punctuation">{</span>
    <span class="token string">"sync_status"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span> <span class="token string">"863aca2c-65b4-480a-90ae-af160129abbd"</span><span class="token punctuation">:</span> <span class="token string">"ok"</span> <span class="token punctuation">}</span>
<span class="token punctuation">}</span>
</code></pre>
<blockquote>
<p>An example of a multiple requests sync return value:</p>
</blockquote>
<pre><code><span class="token punctuation">{</span>
    <span class="token string">"sync_status"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span>
        <span class="token string">"32eaa699-e9d7-47ed-91ea-e58d475791f1"</span><span class="token punctuation">:</span> <span class="token string">"ok"</span><span class="token punctuation">,</span>
        <span class="token string">"bec5b356-3cc1-462a-9887-fe145e3e1ebf"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span>
            <span class="token string">"error_code"</span><span class="token punctuation">:</span> <span class="token number">15</span><span class="token punctuation">,</span>
            <span class="token string">"error"</span><span class="token punctuation">:</span> <span class="token string">"Invalid temporary id"</span>
        <span class="token punctuation">}</span>
    <span class="token punctuation">}</span>
<span class="token punctuation">}</span>
</code></pre>
<blockquote>
<p>An example of an error with additional context in <code>error_extra</code>:</p>
</blockquote>
<pre><code><span class="token punctuation">{</span>
    <span class="token string">"sync_status"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span>
        <span class="token string">"bec5b356-3cc1-462a-9887-fe145e3e1ebf"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span>
            <span class="token string">"error_tag"</span><span class="token punctuation">:</span> <span class="token string">"INVALID_ARGUMENT_VALUE"</span><span class="token punctuation">,</span>
            <span class="token string">"error_code"</span><span class="token punctuation">:</span> <span class="token number">20</span><span class="token punctuation">,</span>
            <span class="token string">"error"</span><span class="token punctuation">:</span> <span class="token string">"Invalid argument value"</span><span class="token punctuation">,</span>
            <span class="token string">"http_code"</span><span class="token punctuation">:</span> <span class="token number">400</span><span class="token punctuation">,</span>
            <span class="token string">"error_extra"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span>
                <span class="token string">"argument"</span><span class="token punctuation">:</span> <span class="token string">"file_url"</span><span class="token punctuation">,</span>
                <span class="token string">"explanation"</span><span class="token punctuation">:</span> <span class="token string">"file_url contains disallowed URL"</span>
            <span class="token punctuation">}</span>
        <span class="token punctuation">}</span>
    <span class="token punctuation">}</span>
<span class="token punctuation">}</span>
</code></pre>
<p>The error object may contain the following fields:</p>
<table>
<thead>
<tr>
<th>Field</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>error_tag <em>String</em></td>
<td>A machine-readable error identifier (e.g., <code>INVALID_ARGUMENT_VALUE</code>).</td>
</tr>
<tr>
<td>error_code <em>Integer</em></td>
<td>A numeric error code.</td>
</tr>
<tr>
<td>error <em>String</em></td>
<td>A human-readable error message.</td>
</tr>
<tr>
<td>http_code <em>Integer</em></td>
<td>The HTTP status code associated with this error.</td>
</tr>
<tr>
<td>error_extra <em>Object</em></td>
<td>Additional context about the error. Contents vary by error type; common fields are documented below.</td>
</tr>
</tbody></table>
<p>Common fields in <code>error_extra</code>:</p>
<table>
<thead>
<tr>
<th>Field</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>argument <em>String</em></td>
<td>The name of the argument that caused the error.</td>
</tr>
<tr>
<td>explanation <em>String</em></td>
<td>A detailed error description, included when it provides more context than the generic <code>error</code> message.</td>
</tr>
<tr>
<td>retry_after <em>Integer</em></td>
<td>Seconds to wait before retrying (for rate-limited requests).</td>
</tr>
<tr>
<td>workspace_id <em>Integer</em></td>
<td>The workspace ID related to the error.</td>
</tr>
<tr>
<td>max_count <em>Integer</em></td>
<td>The limit that was exceeded (for limit-related errors).</td>
</tr>
<tr>
<td>event_id <em>String</em></td>
<td>An event ID for error tracking/support purposes.</td>
</tr>
<tr>
<td>project_id <em>String</em></td>
<td>The project ID related to the error.</td>
</tr>
<tr>
<td>section_id <em>String</em></td>
<td>The section ID related to the error.</td>
</tr>
<tr>
<td>bad_item <em>Object</em></td>
<td>Information about the item that caused the error.</td>
</tr>
</tbody></table>
<p>The result of command executions will be stored in the following response JSON
object field:</p>
<table>
<thead>
<tr>
<th>Data</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>temp_id_mapping <em>Object</em></td>
<td>A dictionary object that maps temporary resource IDs to real resource IDs.</td>
</tr>
<tr>
<td>sync_status <em>Object</em></td>
<td>A dictionary object containing result of each command execution. The key will be the command&#39;s <code>uuid</code> field and the value will be the result status of the command execution.</td>
</tr>
</tbody></table>
<p>The status result of each command execution is in the <code>sync_status</code> dictionary
object. The key is a command <code>uuid</code> and the value will be the result status of
the command execution.</p>
<p>There are two possible values for each command status:</p>
<ul>
<li>An &quot;ok&quot; string which signals success of the command</li>
<li>An error object containing error information of a command.</li>
</ul>
<p>Please see the adjacent code examples for the possible format of the
<code>sync_status</code>.</p>

<p>The server uses the HTTP status codes to indicate the success or failure of a
request. As is customary in web servers, a 2xx code indicates - success, a
4xx code - an error due to incorrect user provided information, and a 5xx code -
an internal, possibly temporary, error.</p>
<table>
<thead>
<tr>
<th>Status code</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>200 OK</td>
<td>The request was processed successfully.</td>
</tr>
<tr>
<td>400 Bad Request</td>
<td>The request was incorrect.</td>
</tr>
<tr>
<td>401 Unauthorized</td>
<td>Authentication is required, and has failed, or has not yet been provided.</td>
</tr>
<tr>
<td>403 Forbidden</td>
<td>The request was valid, but for something that is forbidden.</td>
</tr>
<tr>
<td>404 Not Found</td>
<td>The requested resource could not be found.</td>
</tr>
<tr>
<td>429 Too Many Requests</td>
<td>The user has sent too many requests in a given amount of time.</td>
</tr>
<tr>
<td>500 Internal Server Error</td>
<td>The request failed due to a server error.</td>
</tr>
<tr>
<td>503 Service Unavailable</td>
<td>The server is currently unable to handle the request.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example of batching multiple commands:</p>
</blockquote>
<pre><code class="language-shell"><span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
  <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
  <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
  {
    "type": "project_add",
    "temp_id": "0a57a3db-2ff1-4d2d-adf6-12490c13c712",
    "uuid": "2c0f6e03-c372-46ba-8e85-d94af56abcf3",
    "args": { "name": "Shopping List" }
  },
  {
    "type": "item_add",
    "temp_id": "ef3d840e-84c9-4433-9a32-86ae9a1e7d42",
    "uuid": "49ede211-12f3-42e9-8345-4c0d2b29c08d",
    "args": { "content": "Buy Milk", "project_id": "0a57a3db-2ff1-4d2d-adf6-12490c13c712" }
  },
  {
    "type": "item_add",
    "temp_id": "8a23c8cb-1d76-469d-a2c0-80a28b3ea6f6",
    "uuid": "46619250-ae02-4ab0-bd31-3c9ab0307e53",
    "args": { "content": "Buy Coffee", "project_id": "0a57a3db-2ff1-4d2d-adf6-12490c13c712" }
  },
  {
    "type": "item_add",
    "temp_id": "bf087eaf-aea9-4cb1-ab57-85188a2d428f",
    "uuid": "d0a1666b-d615-4250-aac5-65c7ea89091a",
    "args": { "content": "Buy Sugar", "project_id": "0a57a3db-2ff1-4d2d-adf6-12490c13c712" }
  }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
    <span class="token string">"2c0f6e03-c372-46ba-8e85-d94af56abcf3"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span>,
    <span class="token string">"49ede211-12f3-42e9-8345-4c0d2b29c08d"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span>,
    <span class="token string">"d0a1666b-d615-4250-aac5-65c7ea89091a"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span>,
    <span class="token string">"46619250-ae02-4ab0-bd31-3c9ab0307e53"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span>
  <span class="token punctuation">}</span>,
  <span class="token string">"temp_id_mapping"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
    <span class="token string">"8a23c8cb-1d76-469d-a2c0-80a28b3ea6f6"</span><span class="token builtin class-name">:</span> <span class="token string">"6X6HrfVQvQq5WCXH"</span>,
    <span class="token string">"0a57a3db-2ff1-4d2d-adf6-12490c13c712"</span><span class="token builtin class-name">:</span> <span class="token string">"6X6HrhXfQ9857XVG"</span>,
    <span class="token string">"bf087eaf-aea9-4cb1-ab57-85188a2d428f"</span><span class="token builtin class-name">:</span> <span class="token string">"6X6HrjHFgc3jQM8H"</span>,
    <span class="token string">"ef3d840e-84c9-4433-9a32-86ae9a1e7d42"</span><span class="token builtin class-name">:</span> <span class="token string">"6X6HrmjgW88crvMC"</span>
  <span class="token punctuation">}</span>,
  <span class="token string">"full_sync"</span><span class="token builtin class-name">:</span> true,
  <span class="token string">"sync_token"</span><span class="token builtin class-name">:</span> <span class="token string">"GSg4kDBAKWU7TZA_a-gcuSpxmO1lXT5bhLqUGd1F-AH69_qKEdkN_fJoBq3c"</span>
<span class="token punctuation">}</span>
</code></pre>
<p>When working with the Sync API, changes can be <strong>batched</strong> into one commit.
In our example, we&#39;re batching the creation of a &#39;Shopping List&#39; project with three
different items.</p>
<p>As we&#39;ve committed the changes all at once, we’re significantly reducing the amount of
network calls that have to be made, as well as ensuring we’re not running into any rate
limiting issues.</p>

<p>The Sync API allows clients to retrieve only updated resources, and this is done
by using the <code>sync_token</code> in your Sync API request.</p>
<p>On your initial sync request, specify <code>sync_token=*</code> in your request, and all
the user&#39;s active resource data will be returned. The server will also
return a new <code>sync_token</code> in the Sync API response.</p>
<p>In your subsequent Sync request, use the <code>sync_token</code> that you received from
your previous sync response, and the Todoist API server will return only the
updated resource data.</p>
<h3 id="full-sync-data-delay">Full sync data delay</h3>
<p>For big accounts, the data in the initial sync may be returned with some delay,
and newer objects and updates may seem to be missing. The <code>full_sync_date_utc</code>
attribute should be the same or very close to the current UTC date. If you notice a
bigger time difference, it&#39;s recommended to do an incremental sync using the
<code>sync_token</code> included in that initial sync response to get the latest updates.</p>

<blockquote>
<p>An example workspace object:</p>
</blockquote>
<pre><code><span class="token punctuation">{</span>
  <span class="token string">"created_at"</span><span class="token punctuation">:</span> <span class="token string">"2024-10-19T10:00:00.123456Z"</span><span class="token punctuation">,</span>
  <span class="token string">"creator_id"</span><span class="token punctuation">:</span> <span class="token string">"123"</span><span class="token punctuation">,</span>
  <span class="token string">"current_active_projects"</span><span class="token punctuation">:</span> <span class="token number">5</span><span class="token punctuation">,</span>
  <span class="token string">"current_member_count"</span><span class="token punctuation">:</span> <span class="token number">2</span><span class="token punctuation">,</span>
  <span class="token string">"current_template_count"</span><span class="token punctuation">:</span> <span class="token number">0</span><span class="token punctuation">,</span>
  <span class="token string">"description"</span><span class="token punctuation">:</span> <span class="token string">"Workspace description"</span><span class="token punctuation">,</span>
  <span class="token string">"desktop_workspace_modal"</span><span class="token punctuation">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
  <span class="token string">"domain_discovery"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
  <span class="token string">"domain_name"</span><span class="token punctuation">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
  <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"1234"</span><span class="token punctuation">,</span>
  <span class="token string">"invite_code"</span><span class="token punctuation">:</span> <span class="token string">"ptoh4SICUu4"</span><span class="token punctuation">,</span>
  <span class="token string">"is_collapsed"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
  <span class="token string">"is_deleted"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
  <span class="token string">"is_guest_allowed"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
  <span class="token string">"is_link_sharing_enabled"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
  <span class="token string">"is_trial_pending"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
  <span class="token string">"limits"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span>
    <span class="token string">"current"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span>
      <span class="token string">"admin_tools"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
      <span class="token string">"advanced_permissions"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
      <span class="token string">"automatic_backups"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
      <span class="token string">"calendar_layout"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
      <span class="token string">"durations"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
      <span class="token string">"max_collaborators"</span><span class="token punctuation">:</span> <span class="token number">250</span><span class="token punctuation">,</span>
      <span class="token string">"max_folders_per_workspace"</span><span class="token punctuation">:</span> <span class="token number">1000</span><span class="token punctuation">,</span>
      <span class="token string">"max_guests_per_workspace"</span><span class="token punctuation">:</span> <span class="token number">1000</span><span class="token punctuation">,</span>
      <span class="token string">"max_projects"</span><span class="token punctuation">:</span> <span class="token number">5</span><span class="token punctuation">,</span>
      <span class="token string">"max_workspace_templates"</span><span class="token punctuation">:</span> <span class="token number">100</span><span class="token punctuation">,</span>
      <span class="token string">"max_workspace_users"</span><span class="token punctuation">:</span> <span class="token number">1000</span><span class="token punctuation">,</span>
      <span class="token string">"max_workspaces"</span><span class="token punctuation">:</span> <span class="token number">50</span><span class="token punctuation">,</span>
      <span class="token string">"plan_name"</span><span class="token punctuation">:</span> <span class="token string">"teams_workspaces_starter"</span><span class="token punctuation">,</span>
      <span class="token string">"reminders"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
      <span class="token string">"reminders_at_due"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
      <span class="token string">"security_controls"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
      <span class="token string">"team_activity"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
      <span class="token string">"team_activity_plus"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
      <span class="token string">"upload_limit_mb"</span><span class="token punctuation">:</span> <span class="token number">5</span>
    <span class="token punctuation">}</span><span class="token punctuation">,</span>
    <span class="token string">"next"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span>
      <span class="token string">"admin_tools"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
      <span class="token string">"advanced_permissions"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
      <span class="token string">"automatic_backups"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
      <span class="token string">"max_collaborators"</span><span class="token punctuation">:</span> <span class="token number">250</span><span class="token punctuation">,</span>
      <span class="token string">"max_guests_per_workspace"</span><span class="token punctuation">:</span> <span class="token number">1000</span><span class="token punctuation">,</span>
      <span class="token string">"max_projects"</span><span class="token punctuation">:</span> <span class="token number">1000</span><span class="token punctuation">,</span>
      <span class="token string">"max_workspace_users"</span><span class="token punctuation">:</span> <span class="token number">1000</span><span class="token punctuation">,</span>
      <span class="token string">"plan_name"</span><span class="token punctuation">:</span> <span class="token string">"teams_workspaces_business"</span><span class="token punctuation">,</span>
      <span class="token string">"reminders"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
      <span class="token string">"security_controls"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
      <span class="token string">"upload_limit_mb"</span><span class="token punctuation">:</span> <span class="token number">100</span>
    <span class="token punctuation">}</span>
  <span class="token punctuation">}</span><span class="token punctuation">,</span>
  <span class="token string">"logo_big"</span><span class="token punctuation">:</span> <span class="token string">"https://..."</span><span class="token punctuation">,</span>
  <span class="token string">"logo_medium"</span><span class="token punctuation">:</span> <span class="token string">"https://..."</span><span class="token punctuation">,</span>
  <span class="token string">"logo_s640"</span><span class="token punctuation">:</span> <span class="token string">"https://..."</span><span class="token punctuation">,</span>
  <span class="token string">"logo_small"</span><span class="token punctuation">:</span> <span class="token string">"https://..."</span><span class="token punctuation">,</span>
  <span class="token string">"member_count_by_type"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span>
    <span class="token string">"admin_count"</span><span class="token punctuation">:</span> <span class="token number">2</span><span class="token punctuation">,</span>
    <span class="token string">"guest_count"</span><span class="token punctuation">:</span> <span class="token number">0</span><span class="token punctuation">,</span>
    <span class="token string">"member_count"</span><span class="token punctuation">:</span> <span class="token number">0</span>
  <span class="token punctuation">}</span><span class="token punctuation">,</span>
  <span class="token string">"name"</span><span class="token punctuation">:</span> <span class="token string">"Workspace name"</span><span class="token punctuation">,</span>
  <span class="token string">"pending_invitations"</span><span class="token punctuation">:</span> <span class="token punctuation">[</span>
    <span class="token string">"pending@doist.com"</span>
  <span class="token punctuation">]</span><span class="token punctuation">,</span>
  <span class="token string">"pending_invites_by_type"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span>
    <span class="token string">"admin_count"</span><span class="token punctuation">:</span> <span class="token number">1</span><span class="token punctuation">,</span>
    <span class="token string">"guest_count"</span><span class="token punctuation">:</span> <span class="token number">0</span><span class="token punctuation">,</span>
    <span class="token string">"member_count"</span><span class="token punctuation">:</span> <span class="token number">0</span>
  <span class="token punctuation">}</span><span class="token punctuation">,</span>
  <span class="token string">"plan"</span><span class="token punctuation">:</span> <span class="token string">"STARTER"</span><span class="token punctuation">,</span>
  <span class="token string">"properties"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span><span class="token punctuation">}</span><span class="token punctuation">,</span>
  <span class="token string">"restrict_email_domains"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
  <span class="token string">"role"</span><span class="token punctuation">:</span> <span class="token string">"MEMBER"</span>
<span class="token punctuation">}</span>
</code></pre>
<h4 id="properties">Properties</h4>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>The ID of the workspace.</td>
</tr>
<tr>
<td>name <em>String</em></td>
<td>The name of the workspace (up to 255 characters).</td>
</tr>
<tr>
<td>description <em>String</em></td>
<td>The description of the workspace.</td>
</tr>
<tr>
<td>plan <em>String</em></td>
<td>The subscription plan this workspace is currently on, either <code>STARTER</code> or <code>BUSINESS</code>.</td>
</tr>
<tr>
<td>is_link_sharing_enabled <em>Boolean</em></td>
<td>True if users are allowed to join the workspace using an invitation link. Default value is True. <em>For guests, this field will be set to <code>null</code> as guests are not allowed to have access to this field.</em></td>
</tr>
<tr>
<td>is_guest_allowed <em>Boolean</em></td>
<td>True if users from outside the workspace are allowed to join or be invited to workspace projects. Default value is True.</td>
</tr>
<tr>
<td>invite_code <em>String</em></td>
<td>The invitation code used to generate an invitation link. If <code>is_link_sharing_enabled</code> is True, anyone can join the workspace using this code. <em>For guests, this field will be set to <code>null</code> as guests are not allowed to have access to this field.</em></td>
</tr>
<tr>
<td>role <em>String</em></td>
<td>The role of the requesting user in this workspace. Possible values are: <code>ADMIN</code>, <code>MEMBER</code> or <code>GUEST</code>. A guest is someone who is a collaborator of a workspace project, without being an actual member of the workspace. This field can be <code>null</code> if the requesting user is not part of the workspace. For example, when receiving the workspace deletion related sync update when a user leaves or is removed from a workspace.</td>
</tr>
<tr>
<td>logo_big <em>String</em></td>
<td>The URL for the big workspace logo image.</td>
</tr>
<tr>
<td>logo_medium <em>String</em></td>
<td>The URL for the medium workspace logo image.</td>
</tr>
<tr>
<td>logo_small <em>String</em></td>
<td>The URL for the small workspace logo image.</td>
</tr>
<tr>
<td>logo_s640 <em>String</em></td>
<td>The URL for the square 640px workspace logo image.</td>
</tr>
<tr>
<td>limits <em>Object</em></td>
<td>A list of restrictions for the workspace based on it&#39;s current plan, denoting what features are enabled and limits are imposed.</td>
</tr>
<tr>
<td>creator_id <em>String</em></td>
<td>The ID of the user who created the workspace.</td>
</tr>
<tr>
<td>created_at <em>String</em></td>
<td>The date when the workspace was created.</td>
</tr>
<tr>
<td>is_deleted <em>Boolean</em></td>
<td>True if it is a deleted workspace.</td>
</tr>
<tr>
<td>is_collapsed <em>Boolean</em></td>
<td>True if the workspace is collapsed. This is a user-specific attribute and will reflect the requesting user’s <code>is_collapsed</code> state.</td>
</tr>
<tr>
<td>domain_name <em>String</em></td>
<td>The domain name of the workspace.</td>
</tr>
<tr>
<td>domain_discovery <em>Boolean</em></td>
<td>True if users with e-mail addresses in the workspace domain can join the workspace without an invitation.</td>
</tr>
<tr>
<td>restrict_email_domains <em>Boolean</em></td>
<td>True if only users with e-mail addresses in the workspace domain can join the workspace.</td>
</tr>
<tr>
<td>properties <em>Object</em></td>
<td>Configuration properties for the workspace. See <a href="#workspace-properties">Workspace Properties</a> below for detailed structure.</td>
</tr>
<tr>
<td>default_collaborators <em>Object</em></td>
<td>Default collaborators that are automatically added to new projects in this workspace. Contains <code>user_ids</code> (array of user IDs) and <code>predefined_group_ids</code> (array of predefined group names).</td>
</tr>
<tr>
<td>desktop_workspace_modal <em>String</em></td>
<td>Enum value indicating when desktop should show workspace modal. Currently only supports <code>TRIAL_OFFER</code> for trial offers. <code>null</code> when no modal should be shown. This field is automatically set by the backend when mobile devices are registered and trial eligibility criteria are met.</td>
</tr>
</tbody></table>
<h3 id="workspace-properties">Workspace Properties</h3>
<p>The <code>properties</code> object contains configuration settings for the workspace:</p>
<table>
<thead>
<tr>
<th>Property</th>
<th>Type</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>industry</td>
<td><em>String</em></td>
<td>The industry of the workspace. Possible values: <code>agriculture</code>, <code>arts_entertainment</code>, <code>automotive</code>, <code>banking_financial_services</code>, <code>construction</code>, <code>consulting</code>, <code>consumer_goods</code>, <code>education</code>, <code>energy_utilities</code>, <code>food_beverages</code>, <code>government_public_sector</code>, <code>healthcare_life_sciences</code>, <code>information_technology</code>, <code>insurance</code>, <code>legal_services</code>, <code>manufacturing</code>, <code>media_communications</code>, <code>non_profit</code>, <code>pharmaceuticals</code>, <code>real_estate</code>, <code>retail_wholesale</code>, <code>telecommunications</code>, <code>transportation_logistics</code>, <code>travel_hospitality</code>, <code>other</code>.</td>
</tr>
<tr>
<td>department</td>
<td><em>String</em></td>
<td>The department of the workspace. Possible values: <code>administration</code>, <code>customer_service</code>, <code>finance_accounting</code>, <code>human_resources</code>, <code>information_technology</code>, <code>legal</code>, <code>marketing</code>, <code>operations</code>, <code>product_development</code>, <code>research_development</code>, <code>sales</code>, <code>supply_chain_management</code>, <code>engineering</code>, <code>quality_assurance</code>, <code>executive_management</code>, <code>other</code>.</td>
</tr>
<tr>
<td>organization_size</td>
<td><em>String</em></td>
<td>The size of the organization. Possible values: <code>size_1</code>, <code>size_2_to_10</code>, <code>size_11_to_50</code>, <code>size_51_to_100</code>, <code>size_101_to_250</code>, <code>size_51_to_250</code>, <code>more_than_250</code>.</td>
</tr>
<tr>
<td>creator_role</td>
<td><em>String</em></td>
<td>The role of the workspace creator. Possible values: <code>owner_founder</code>, <code>leader</code>, <code>individual_contributor</code>.</td>
</tr>
<tr>
<td>region</td>
<td><em>String</em></td>
<td>2 digit continent code. Possible values: <code>AF</code>, <code>AS</code>, <code>EU</code>, <code>NA</code>, <code>SA</code>, <code>OC</code>, <code>AN</code>.</td>
</tr>
<tr>
<td>country</td>
<td><em>String</em></td>
<td>2 digit ISO 3166-1 alpha-2 country code.</td>
</tr>
<tr>
<td>default_access_level</td>
<td><em>String</em></td>
<td>Default access level for new projects in the workspace. Possible values: <code>restricted</code>, <code>team</code> (default).</td>
</tr>
<tr>
<td>beta_enabled</td>
<td><em>Boolean</em></td>
<td>Indicates whether beta features are enabled for this workspace. Default value is <code>false</code>.</td>
</tr>
<tr>
<td>acquisition_source</td>
<td><em>String</em></td>
<td>The marketing channel or source that led to workspace creation. Possible values: <code>high_paid_channel</code></td>
</tr>
<tr>
<td>hdyhau</td>
<td><em>String</em></td>
<td>How did you hear about us - marketing attribution field. Possible values: <code>friend</code>, <code>social_media</code>, <code>ai_chatbot</code>, <code>search_engine</code>, <code>app_store</code>, <code>other</code></td>
</tr>
</tbody></table>

<blockquote>
<p>Example add workspace request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "workspace_add",
        "temp_id": "4ff1e388-5ca6-453a-b0e8-662ebf373b6b",
        "uuid": "32774db9-a1da-4550-8d9d-910372124fa4",
        "args": {
            "name": "Fellowship Workspace"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"32774db9-a1da-4550-8d9d-910372124fa4"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token string">"temp_id_mapping"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"4ff1e388-5ca6-453a-b0e8-662ebf373b6b"</span><span class="token builtin class-name">:</span> <span class="token string">"6X6WMG4rmqx6FXQ9"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Add a new workspace.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>name <em>String</em></td>
<td>Yes</td>
<td>The name of the workspace.</td>
</tr>
<tr>
<td>description <em>String</em></td>
<td>No</td>
<td>The description of the workspace (up to 1024 characters).</td>
</tr>
<tr>
<td>is_link_sharing_enabled <em>Boolean</em></td>
<td>No</td>
<td>Indicates if users are allowed to join the workspace using an invitation link. Default value is True.</td>
</tr>
<tr>
<td>is_guest_allowed <em>Boolean</em></td>
<td>No</td>
<td>Indicates if users from outside the workspace are allowed to join or be invited to workspace projects. Default value is True.</td>
</tr>
<tr>
<td>domain_name <em>String</em></td>
<td>No</td>
<td>The domain name of the workspace.</td>
</tr>
<tr>
<td>domain_discovery <em>Boolean</em></td>
<td>No</td>
<td>True if users with e-mail addresses in the workspace domain can join the workspace without an invitation.</td>
</tr>
<tr>
<td>restrict_email_domains <em>Boolean</em></td>
<td>No</td>
<td>True if only users with e-mail addresses in the workspace domain can join the workspace.</td>
</tr>
<tr>
<td>properties <em>Object</em></td>
<td>No</td>
<td>Configuration properties for the workspace. See <a href="#workspace-properties">Workspace Properties</a> for detailed structure.</td>
</tr>
<tr>
<td>default_collaborators <em>Object</em></td>
<td>No</td>
<td>Default collaborators for new projects. Object with <code>user_ids</code> (array of integers) and <code>predefined_group_ids</code> (array of strings). If not provided or set to <code>null</code> then by default all workspace members are added as the default collaborators.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example update workspace request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "workspace_update",
        "temp_id": "4ff1e388-5ca6-453a-b0e8-662ebf373b6b",
        "uuid": "32774db9-a1da-4550-8d9d-910372124fa4",
        "args": {
            "id": "12345",
            "description": "Where magic happens"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"32774db9-a1da-4550-8d9d-910372124fa4"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token string">"temp_id_mapping"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"4ff1e388-5ca6-453a-b0e8-662ebf373b6b"</span><span class="token builtin class-name">:</span> <span class="token string">"6X6WMMqgq2PWxjCX"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Update an existing workspace.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>Real or temp ID of the workspace</td>
</tr>
<tr>
<td>name <em>String</em></td>
<td>No</td>
<td>The name of the workspace.</td>
</tr>
<tr>
<td>description <em>String</em></td>
<td>No</td>
<td>The description of the workspace (up to 1024 characters).</td>
</tr>
<tr>
<td>is_collapsed <em>Boolean</em></td>
<td>No</td>
<td>The collapsed state of the workspace for the current user</td>
</tr>
<tr>
<td>is_link_sharing_enabled <em>Boolean</em></td>
<td>No</td>
<td>Indicates if users are allowed to join the workspace using an invitation link.</td>
</tr>
<tr>
<td>is_guest_allowed <em>Boolean</em></td>
<td>No</td>
<td>Indicates if users from outside the workspace are allowed to join or be invited to workspace projects. Default value is True.</td>
</tr>
<tr>
<td>invite_code <em>String</em></td>
<td>No</td>
<td>Regenerate the invite_code for the workspace. Any non-empty string value will regenerate a new code, the provided value with this argument is not significant, only an indication to regenerate the code.</td>
</tr>
<tr>
<td>domain_name <em>String</em></td>
<td>No</td>
<td>The domain name of the workspace.</td>
</tr>
<tr>
<td>domain_discovery <em>Boolean</em></td>
<td>No</td>
<td>True if users with e-mail addresses in the workspace domain can join the workspace without an invitation.</td>
</tr>
<tr>
<td>restrict_email_domains <em>Boolean</em></td>
<td>No</td>
<td>True if only users with e-mail addresses in the workspace domain can join the workspace.</td>
</tr>
<tr>
<td>properties <em>Object</em></td>
<td>No</td>
<td>Configuration properties for the workspace. See <a href="#workspace-properties">Workspace Properties</a> for detailed structure.</td>
</tr>
<tr>
<td>default_collaborators <em>Object</em></td>
<td>No</td>
<td>Default collaborators for new projects. Object with <code>user_ids</code> (array of integers) and <code>predefined_group_ids</code> (array of strings). If not provided or set to <code>null</code> then by default all workspace members are added as the default collaborators.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example leave workspace request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "workspace_leave",
        "temp_id": "4ff1e388-5ca6-453a-b0e8-662ebf373b6b",
        "uuid": "32774db9-a1da-4550-8d9d-910372124fa4",
        "args": {
            "id": "6X6WMMqgq2PWxjCX",
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"32774db9-a1da-4550-8d9d-910372124fa4"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Remove self from a workspace. The user is also removed from any workspace project previously joined.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>Real or temp ID of the workspace</td>
</tr>
</tbody></table>
<p><em>All workspace_users can leave a workspace by themselves.</em></p>

<blockquote>
<p>Example delete workspace request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "workspace_delete",
        "temp_id": "4ff1e388-5ca6-453a-b0e8-662ebf373b6b",
        "uuid": "32774db9-a1da-4550-8d9d-910372124fa4",
        "args": {
            "id": "6X6WMRPC43g2gHVx"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"32774db9-a1da-4550-8d9d-910372124fa4"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Delete an existing workspace.</p>
<p><em>This command is only usable by workspace admins. Other users will get a “forbidden” error.</em></p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>The ID of the workspace</td>
</tr>
</tbody></table>

<p><strong><code>workspace_users</code> are not returned in full sync responses, only in incremental sync</strong>. To keep a list of workspace users up-to-date, clients should first <a href="#tag/Workspace/operation/get_workspaces_users_api_v1_workspaces_users_get">list all workspace users</a>, then use incremental sync to update that initial list as needed.</p>
<p><code>workspace_users</code> are not the same as collaborators. Two users can be members of a common workspace without having a common shared project, so they will both “see” each other in <code>workspace_users</code> but not in collaborators.</p>
<p>Guests will not receive <code>workspace_users</code> sync events or resources.</p>
<blockquote>
<p>An example workspace_users object:</p>
</blockquote>
<pre><code><span class="token punctuation">{</span>
    <span class="token string">"user_id"</span><span class="token punctuation">:</span> <span class="token string">"1855581"</span><span class="token punctuation">,</span>
    <span class="token string">"workspace_id"</span><span class="token punctuation">:</span> <span class="token string">"424876"</span><span class="token punctuation">,</span>
    <span class="token string">"user_email"</span><span class="token punctuation">:</span> <span class="token string">"you@example.com"</span><span class="token punctuation">,</span>
    <span class="token string">"full_name"</span><span class="token punctuation">:</span> <span class="token string">"Example User"</span><span class="token punctuation">,</span>
    <span class="token string">"timezone"</span><span class="token punctuation">:</span> <span class="token string">"GMT +3:00"</span><span class="token punctuation">,</span>
    <span class="token string">"avatar_big"</span><span class="token punctuation">:</span> <span class="token string">"https://*.cloudfront.net/*_big.jpg"</span><span class="token punctuation">,</span>
    <span class="token string">"avatar_medium"</span><span class="token punctuation">:</span> <span class="token string">"https://*.cloudfront.net/*_medium.jpg"</span><span class="token punctuation">,</span>
    <span class="token string">"avatar_s640"</span><span class="token punctuation">:</span> <span class="token string">"https://*.cloudfront.net/*_s640.jpg"</span><span class="token punctuation">,</span>
    <span class="token string">"avatar_small"</span><span class="token punctuation">:</span> <span class="token string">"https://*.cloudfront.net/*_small.jpg"</span><span class="token punctuation">,</span>
    <span class="token string">"image_id"</span><span class="token punctuation">:</span> <span class="token string">"d160009dfd52b991030d55227003450f"</span><span class="token punctuation">,</span>
    <span class="token string">"role"</span><span class="token punctuation">:</span> <span class="token string">"MEMBER"</span>
    <span class="token string">"is_deleted"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
<span class="token punctuation">}</span>
</code></pre>
<h4 id="properties">Properties</h4>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>user_id <em>String</em></td>
<td>The user ID.</td>
</tr>
<tr>
<td>workspace_id <em>String</em></td>
<td>The workspace ID for this user.</td>
</tr>
<tr>
<td>user_email <em>String</em></td>
<td>The user email.</td>
</tr>
<tr>
<td>full_name <em>String</em></td>
<td>The full name of the user.</td>
</tr>
<tr>
<td>timezone <em>String</em></td>
<td>The timezone of the user.</td>
</tr>
<tr>
<td>image_id <em>String</em></td>
<td>The ID of the user&#39;s avatar.</td>
</tr>
<tr>
<td>role <em>String</em></td>
<td>The role of the user in this workspace. Possible values are: ADMIN, MEMBER, GUEST. A guest is someone who is a collaborator of a workspace project, without being an actual member of the workspace.</td>
</tr>
<tr>
<td>avatar_big <em>String</em></td>
<td>The link to a 195x195 pixels image of the user&#39;s avatar.</td>
</tr>
<tr>
<td>avatar_medium <em>String</em></td>
<td>The link to a 60x60 pixels image of the user&#39;s avatar.</td>
</tr>
<tr>
<td>avatar_s640 <em>String</em></td>
<td>The link to a 640x640 pixels image of the user&#39;s avatar.</td>
</tr>
<tr>
<td>avatar_small <em>String</em></td>
<td>The link to a 35x35 pixels image of the user&#39;s avatar.</td>
</tr>
<tr>
<td>is_deleted <em>Boolean</em></td>
<td>Whether the workspace user is marked as deleted.</td>
</tr>
</tbody></table>
<p>Avatar URLs are only available if the user has an avatar, in other words, when the <code>image_id</code> is not <code>null</code>.</p>

<blockquote>
<p>Example role change for a workspace user request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "workspace_update_user",
        "temp_id": "4ff1e388-5ca6-453a-b0e8-662ebf373b6b",
        "uuid": "32774db9-a1da-4550-8d9d-910372124fa4",
        "args": {
            "workspace_id": "12345,
            "user_email": "user@acme.com",
            "role": "ADMIN"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"32774db9-a1da-4550-8d9d-910372124fa4"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token string">"temp_id_mapping"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"4ff1e388-5ca6-453a-b0e8-662ebf373b6b"</span><span class="token builtin class-name">:</span> <span class="token string">"12345"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Change the role of a workspace user.</p>
<p><em>Admins or members can not be downgraded to guests.</em></p>
<p><em>This command is only usable by workspace admins. Other users will get a “forbidden” error.</em></p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>Real or temp ID of the workspace</td>
</tr>
<tr>
<td>user_email <em>String</em></td>
<td>Yes</td>
<td>The new member&#39;s email</td>
</tr>
<tr>
<td>role <em>String</em></td>
<td>Yes</td>
<td>The role to be assigned to the new member. Valid values are <code>GUEST</code>, <code>MEMBER</code> and <code>ADMIN</code>.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example sidebar preference update for a workspace user request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "workspace_update_user_sidebar_preference",
        "temp_id": "4ff1e388-5ca6-453a-b0e8-662ebf373b6b",
        "uuid": "32774db9-a1da-4550-8d9d-910372124fa4",
        "args": {
            "workspace_id": "12345",
            "sidebar_preference": "A_TO_Z"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"32774db9-a1da-4550-8d9d-910372124fa4"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token string">"temp_id_mapping"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"4ff1e388-5ca6-453a-b0e8-662ebf373b6b"</span><span class="token builtin class-name">:</span> <span class="token string">"12345"</span><span class="token punctuation">}</span>,
  <span class="token string">"workspaces"</span><span class="token builtin class-name">:</span> <span class="token punctuation">[</span>
    <span class="token punctuation">{</span>
      <span class="token string">"id"</span><span class="token builtin class-name">:</span> <span class="token string">"12345"</span>,
      <span class="token string">"sidebar_preference"</span><span class="token builtin class-name">:</span> <span class="token string">"A_TO_Z"</span>,
      <span class="token punctuation">..</span>.
    <span class="token punctuation">}</span>
  <span class="token punctuation">]</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Update the sidebar preference for the requesting user in a workspace. This defines the order projects and folders are sorted in the Workspace Overview and Sidebar.</p>
<p><em>Any workspace user can update their own sidebar preference.</em></p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>workspace_id <em>String</em></td>
<td>Yes</td>
<td>Real or temp ID of the workspace</td>
</tr>
<tr>
<td>sidebar_preference <em>String</em></td>
<td>Yes</td>
<td>The sidebar preference. Valid values are <code>MANUAL</code>, <code>A_TO_Z</code>, and <code>Z_TO_A</code>.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example delete workspace user request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "workspace_delete_user",
        "temp_id": "4ff1e388-5ca6-453a-b0e8-662ebf373b6b",
        "uuid": "32774db9-a1da-4550-8d9d-910372124fa4",
        "args": {
            "workspace_id": "12345",
            "user_email": "user@acme.com"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"32774db9-a1da-4550-8d9d-910372124fa4"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Remove a user from a workspace. That user is also removed from any workspace project they joined.</p>
<p><em>This command is only usable by workspace admins. Other users will get a “forbidden” error.</em></p>
<p><em>Admins can use this command to remove themselves from a workspace, unless they are the last admin in the workspace. In that case, a “forbidden” error will be returned.</em></p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>Real or temp ID of the workspace</td>
</tr>
<tr>
<td>user_email <em>String</em></td>
<td>Yes</td>
<td>The email of the member to be deleted</td>
</tr>
</tbody></table>

<blockquote>
<p>Example request to invite users to a workspace through the Sync API:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/sync/v10/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
      {
        "type": "workspace_invite",
        "uuid": "32774db9-a1da-4550-8d9d-910372124fa4",
        "args": {
            "id": "12345",
            "email_list": ["foo@example.com", "bar@example.com"],
            "role": "MEMBER"
        }
      }]
    '</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"32774db9-a1da-4550-8d9d-910372124fa4"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>This will create workspace invitations for a list of email addresses. Usable by all workspace members and admins.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>ID of the workspace.</td>
</tr>
<tr>
<td>email_list <em>List of String</em></td>
<td>Yes</td>
<td>A list of emails to be invited to the workspace.</td>
</tr>
<tr>
<td>role <em>String</em></td>
<td>No</td>
<td>The role the user will be given if they accept the invite. Possible values are <code>ADMIN</code>, <code>MEMBER</code>, and <code>GUEST</code>. If not provided, the default value according to the plan will be used. For Starter plans, the default is ADMIN and for Business plans, the default is MEMBER.</td>
</tr>
</tbody></table>

<blockquote>
<p>An example view option object:</p>
</blockquote>
<pre><code><span class="token punctuation">{</span>
    <span class="token string">"view_type"</span><span class="token punctuation">:</span> <span class="token string">"project"</span><span class="token punctuation">,</span>
    <span class="token string">"object_id"</span><span class="token punctuation">:</span> <span class="token string">"6Jf8VQXxpwv56VQ7"</span><span class="token punctuation">,</span>
    <span class="token string">"filtered_by"</span><span class="token punctuation">:</span> <span class="token string">"!assigned"</span><span class="token punctuation">,</span>
    <span class="token string">"grouped_by"</span><span class="token punctuation">:</span> <span class="token string">"priority"</span><span class="token punctuation">,</span>
    <span class="token string">"sorted_by"</span><span class="token punctuation">:</span> <span class="token string">"added_date"</span><span class="token punctuation">,</span>
    <span class="token string">"sort_order"</span><span class="token punctuation">:</span> <span class="token string">"asc"</span><span class="token punctuation">,</span>
    <span class="token string">"show_completed_tasks"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"view_mode"</span><span class="token punctuation">:</span> <span class="token string">"calendar"</span><span class="token punctuation">,</span>
    <span class="token string">"calendar_settings"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span> <span class="token string">"layout"</span><span class="token punctuation">:</span> <span class="token string">"month"</span> <span class="token punctuation">}</span><span class="token punctuation">,</span>
    <span class="token string">"is_deleted"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"deadline"</span><span class="token punctuation">:</span> <span class="token string">"no deadline"</span>
<span class="token punctuation">}</span>
</code></pre>
<h4 id="properties">Properties</h4>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>view_type <em>Enum</em></td>
<td>The type of a view customization. <code>today</code> for the today view, <code>upcoming</code> for the upcoming view, <code>project</code> for a project, <code>label</code> for a label, <code>filter</code> for a personal filter or <code>workspace_filter</code> for a team filter.</td>
</tr>
<tr>
<td>object_id <em>String</em></td>
<td>The ID of the object referred to by <code>view_type</code>, when <code>view_type</code> is <code>project</code>, <code>label</code>, <code>filter</code> or <code>workspace_filter</code>.</td>
</tr>
<tr>
<td>filtered_by <em>String</em></td>
<td>A search query for this view customization. <a href="https://www.todoist.com/help/articles/introduction-to-filters-V98wIH">Examples of searches</a> can be found in the Todoist help page.</td>
</tr>
<tr>
<td>grouped_by <em>Enum</em></td>
<td>Grouping criteria for this view customization. One of <code>assignee</code>, <code>added_date</code>, <code>due_date</code>, <code>deadline</code>, <code>label</code>, <code>priority</code>, <code>project</code>, or <code>workspace</code>.</td>
</tr>
<tr>
<td>sorted_by <em>Enum</em></td>
<td>Sorting criteria for this view customization. One of <code>alphabetically</code>, <code>assignee</code>, <code>added_date</code>, <code>due_date</code>, <code>deadline</code>, <code>label</code>, <code>priority</code>, <code>project</code>, <code>workspace</code>, or <code>manual</code>.</td>
</tr>
<tr>
<td>sort_order <em>Enum</em></td>
<td>Sorting order for this view customization. <code>asc</code> for ascending, <code>desc</code> for descending.</td>
</tr>
<tr>
<td>show_completed_tasks <em>Boolean</em></td>
<td>Whether completed tasks should be shown automatically in this view customization.</td>
</tr>
<tr>
<td>view_mode <em>Enum</em></td>
<td>The mode in which to render tasks in this view customization. One of <code>list</code>, <code>board</code>, or <code>calendar</code>. <strong>Note: This setting is ignored in projects, where <code>project.view_style</code> is used instead.</strong></td>
</tr>
<tr>
<td>deadline <em>String</em></td>
<td>A search query for this view customization. <a href="https://www.todoist.com/help/articles/introduction-to-filters-V98wIH">Examples of deadline searches</a> can be found in the Todoist help page.</td>
</tr>
<tr>
<td>calendar_settings <em>JSON</em></td>
<td>The settings for the calendar when <code>view_mode</code> is set to <code>calendar</code>. Currently, only <code>{&quot;layout&quot;: &quot;week&quot;}</code> and <code>{&quot;layout&quot;: &quot;month&quot;}</code> are supported.</td>
</tr>
<tr>
<td>is_deleted <em>Boolean</em></td>
<td>Whether the view option is marked as deleted.</td>
</tr>
</tbody></table>
<p><strong>Note:</strong> <code>view_options.view_mode</code> is secondary to <a href="#tag/Sync/View-Options"><code>project.view_style</code></a> for projects in Todoist clients. The former is set per user, while the latter is set per project.</p>

<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>view_type <em>Enum</em></td>
<td>Yes</td>
<td>Type of the view customization to be set. <code>today</code> for the today view, <code>upcoming</code> for the upcoming view, <code>project</code> for a project, <code>label</code> for a label, <code>filter</code> for a personal filter or <code>workspace_filter</code> for a team filter.</td>
</tr>
<tr>
<td>object_id <em>String</em></td>
<td>Yes</td>
<td>ID of the object referred to by <code>view_type</code>, required when <code>view_type</code> is <code>project</code>, <code>label</code>, <code>filter</code> or <code>workspace_filter</code>.</td>
</tr>
<tr>
<td>filtered_by <em>String</em></td>
<td>No</td>
<td>Search query. <a href="https://www.todoist.com/help/articles/introduction-to-filters-V98wIH">Examples of searches</a> can be found in the Todoist help page.</td>
</tr>
<tr>
<td>grouped_by <em>Enum</em></td>
<td>No</td>
<td>Grouping criteria. One of <code>assignee</code>, <code>added_date</code>, <code>due_date</code>, <code>deadline</code>, <code>label</code>, <code>priority</code>, <code>project</code>, or <code>workspace</code>.</td>
</tr>
<tr>
<td>sorted_by <em>Enum</em></td>
<td>No</td>
<td>Sorting criteria. One of <code>alphabetically</code>, <code>assignee</code>, <code>added_date</code>, <code>due_date</code>, <code>deadline</code>, <code>label</code>, <code>priority</code>, <code>project</code>, <code>workspace</code>, or <code>manual</code>.</td>
</tr>
<tr>
<td>sort_order <em>Enum</em></td>
<td>No</td>
<td>Sorting order. <code>asc</code> for ascending, <code>desc</code> for descending.</td>
</tr>
<tr>
<td>show_completed_tasks <em>Boolean</em></td>
<td>No</td>
<td>Whether completed tasks should be shown automatically in this view customization.</td>
</tr>
<tr>
<td>view_mode <em>Enum</em></td>
<td>No</td>
<td>The mode in which to render tasks. One of <code>list</code>, <code>board</code>, or <code>calendar</code>.</td>
</tr>
<tr>
<td>deadline <em>String</em></td>
<td>No</td>
<td>A search query for this view customization. <a href="https://www.todoist.com/help/articles/introduction-to-filters-V98wIH">Examples of deadline searches</a> can be found in the Todoist help page.</td>
</tr>
<tr>
<td>calendar_settings <em>JSON</em></td>
<td>No</td>
<td>The settings for the calendar when <code>view_mode</code> is set to <code>calendar</code>. Currently, only <code>{&quot;layout&quot;: &quot;week&quot;}</code> and <code>{&quot;layout&quot;: &quot;month&quot;}</code> are supported.</td>
</tr>
</tbody></table>
<blockquote>
<p>Example set view option request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "view_options_set",
        "uuid": "997d4b43-55f1-48a9-9e66-de5785dfd696",
        "args": {
            "view_type": "project",
            "object_id": "6Jf8VQXxpwv56VQ7",
            "view_mode": "board",
            "grouped_by": "assignee"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"997d4b43-55f1-48a9-9e66-de5785dfd696"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>

<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>view_type <em>Enum</em></td>
<td>Yes</td>
<td>Type of the view customization to delete. <code>today</code> for the today view, <code>upcoming</code> for the upcoming view, <code>project</code> for a project, <code>label</code> for a label, or <code>filter</code> for a filter.</td>
</tr>
<tr>
<td>object_id <em>String</em></td>
<td>Yes</td>
<td>ID of the object referred to by <code>view_type</code>, required when <code>view_type</code> is <code>project</code>, <code>label</code>, <code>filter</code> or <code>workspace_filter</code>.</td>
</tr>
</tbody></table>
<blockquote>
<p>Example delete view option request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "view_options_delete",
        "uuid": "f8539c77-7fd7-4846-afad-3b201f0be8a6",
        "args": {
            "view_type": "today"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"f8539c77-7fd7-4846-afad-3b201f0be8a6"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>

<p>Project View Options Defaults (PVODs) define the default view preferences for all users of a project. These settings serve as the baseline view configuration that applies to all project members unless they have their own personal view options set.</p>
<blockquote>
<p>An example Project View Options Default object:</p>
</blockquote>
<pre><code class="language-json"><span class="token punctuation">{</span>
    <span class="token string-property property">"project_id"</span><span class="token operator">:</span> <span class="token string">"2203306141"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"view_mode"</span><span class="token operator">:</span> <span class="token string">"list"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"grouped_by"</span><span class="token operator">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
    <span class="token string-property property">"sorted_by"</span><span class="token operator">:</span> <span class="token string">"due_date"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"sort_order"</span><span class="token operator">:</span> <span class="token string">"asc"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"show_completed_tasks"</span><span class="token operator">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string-property property">"filtered_by"</span><span class="token operator">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
    <span class="token string-property property">"calendar_settings"</span><span class="token operator">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
    <span class="token string-property property">"creator_uid"</span><span class="token operator">:</span> <span class="token number">1855589</span><span class="token punctuation">,</span>
    <span class="token string-property property">"updater_uid"</span><span class="token operator">:</span> <span class="token number">1855589</span>
<span class="token punctuation">}</span>
</code></pre>
<h3 id="properties">Properties</h3>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>project_id</td>
<td>The project ID these defaults apply to (string, required)</td>
</tr>
<tr>
<td>view_mode</td>
<td>The default view mode: <code>list</code>, <code>board</code>, or <code>calendar</code> (string, required)</td>
</tr>
<tr>
<td>grouped_by</td>
<td>How tasks are grouped: <code>due_date</code>, <code>created_at</code>, <code>label</code>, <code>assignee</code>, <code>priority</code>, or <code>project</code> (string or null)</td>
</tr>
<tr>
<td>sorted_by</td>
<td>How tasks are sorted: <code>due_date</code>, <code>created_at</code>, <code>task_order</code>, <code>assignee</code>, <code>alphabetically</code>, or <code>priority</code> (string or null)</td>
</tr>
<tr>
<td>sort_order</td>
<td>Sort direction: <code>asc</code> or <code>desc</code> (string, required)</td>
</tr>
<tr>
<td>show_completed_tasks</td>
<td>Whether to show completed tasks by default (boolean, required)</td>
</tr>
<tr>
<td>filtered_by</td>
<td>JSON string with filter criteria (string or null)</td>
</tr>
<tr>
<td>calendar_settings</td>
<td>Calendar-specific settings when <code>view_mode</code> is <code>calendar</code> (object or null)</td>
</tr>
<tr>
<td>creator_uid</td>
<td>User ID who created these defaults (integer, required)</td>
</tr>
<tr>
<td>updater_uid</td>
<td>User ID who last updated these defaults (integer, required)</td>
</tr>
</tbody></table>
<h3 id="sync-behavior">Sync behavior</h3>
<ul>
<li>PVODs are returned during full sync if the user has access to the project</li>
<li>When a project is created, its PVOD is automatically created and included in the same sync response</li>
<li>Updates to PVODs trigger sync events for all project members</li>
<li>When a PVOD is deleted, a tombstone is returned with <code>is_deleted: true</code> and includes: <code>project_id</code>, <code>is_deleted</code>, <code>creator_uid</code>, <code>updater_uid</code>, <code>show_completed_tasks</code>, and all view option fields (<code>view_mode</code>, <code>grouped_by</code>, <code>sorted_by</code>, <code>sort_order</code>, <code>filtered_by</code>) set to empty strings. <code>calendar_settings</code> is set to <code>null</code></li>
<li>PVODs take precedence over legacy <code>project.view_style</code> settings</li>
</ul>
<h3 id="commands">Commands</h3>
<h4 id="project_view_options_defaults_set">project_view_options_defaults_set</h4>
<p>Updates the default view options for a project. Only users with admin permissions on the project can update PVODs.</p>
<blockquote>
<p>Command arguments:</p>
</blockquote>
<table>
<thead>
<tr>
<th>Name</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>project_id</td>
<td>Yes</td>
<td>The project ID to update defaults for</td>
</tr>
<tr>
<td>view_mode</td>
<td>No</td>
<td>The default view mode: <code>list</code>, <code>board</code>, or <code>calendar</code></td>
</tr>
<tr>
<td>grouped_by</td>
<td>No</td>
<td>How to group tasks (see properties above)</td>
</tr>
<tr>
<td>sorted_by</td>
<td>No</td>
<td>How to sort tasks (see properties above)</td>
</tr>
<tr>
<td>sort_order</td>
<td>No</td>
<td>Sort direction: <code>asc</code> or <code>desc</code></td>
</tr>
<tr>
<td>show_completed_tasks</td>
<td>No</td>
<td>Whether to show completed tasks</td>
</tr>
<tr>
<td>filtered_by</td>
<td>No</td>
<td>JSON string with filter criteria</td>
</tr>
<tr>
<td>calendar_settings</td>
<td>No</td>
<td>Calendar-specific settings (required when <code>view_mode</code> is <code>calendar</code>)</td>
</tr>
</tbody></table>
<blockquote>
<p>Example command:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> <span class="token parameter variable">-X</span> POST <span class="token punctuation">\</span>
    https://api.todoist.com/sync/v9/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Content-Type: application/json"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer <span class="token variable">$token</span>"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token string">'[{
        "type": "project_view_options_defaults_set",
        "uuid": "bf0855a3-0138-44-b618-1cb8d3d7a869",
        "args": {
            "project_id": "2203306141",
            "view_mode": "board",
            "grouped_by": "priority",
            "sorted_by": "due_date",
            "sort_order": "asc",
            "show_completed_tasks": false
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"bf0855a3-0138-44-b618-1cb8d3d7a869"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>

<blockquote>
<p>An example user:</p>
</blockquote>
<pre><code><span class="token punctuation">{</span>
    <span class="token string">"activated_user"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"auto_reminder"</span><span class="token punctuation">:</span> <span class="token number">0</span><span class="token punctuation">,</span>
    <span class="token string">"avatar_big"</span><span class="token punctuation">:</span> <span class="token string">"https://*.cloudfront.net/*_big.jpg"</span><span class="token punctuation">,</span>
    <span class="token string">"avatar_medium"</span><span class="token punctuation">:</span> <span class="token string">"https://*.cloudfront.net/*_medium.jpg"</span><span class="token punctuation">,</span>
    <span class="token string">"avatar_s640"</span><span class="token punctuation">:</span> <span class="token string">"https://*.cloudfront.net/*_s640.jpg"</span><span class="token punctuation">,</span>
    <span class="token string">"avatar_small"</span><span class="token punctuation">:</span> <span class="token string">"https://*.cloudfront.net/*_small.jpg"</span><span class="token punctuation">,</span>
    <span class="token string">"business_account_id"</span><span class="token punctuation">:</span> <span class="token string">"1"</span><span class="token punctuation">,</span>
    <span class="token string">"daily_goal"</span><span class="token punctuation">:</span> <span class="token number">15</span><span class="token punctuation">,</span>
    <span class="token string">"date_format"</span><span class="token punctuation">:</span> <span class="token number">0</span><span class="token punctuation">,</span>
    <span class="token string">"days_off"</span><span class="token punctuation">:</span> <span class="token punctuation">[</span><span class="token number">6</span><span class="token punctuation">,</span> <span class="token number">7</span><span class="token punctuation">]</span><span class="token punctuation">,</span>
    <span class="token string">"email"</span><span class="token punctuation">:</span> <span class="token string">"me@example.com"</span><span class="token punctuation">,</span>
    <span class="token string">"feature_identifier"</span><span class="token punctuation">:</span> <span class="token string">"2671355_0123456789abcdef70123456789abcdefe0123456789abcdefd0123456789abc"</span><span class="token punctuation">,</span>
    <span class="token string">"features"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span>
        <span class="token string">"beta"</span><span class="token punctuation">:</span> <span class="token number">1</span><span class="token punctuation">,</span>
        <span class="token string">"dateist_inline_disabled"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
        <span class="token string">"dateist_lang"</span><span class="token punctuation">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
        <span class="token string">"global.teams"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
        <span class="token string">"has_push_reminders"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
        <span class="token string">"karma_disabled"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
        <span class="token string">"karma_vacation"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
        <span class="token string">"kisa_consent_timestamp"</span><span class="token punctuation">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
        <span class="token string">"restriction"</span><span class="token punctuation">:</span> <span class="token number">3</span>
    <span class="token punctuation">}</span><span class="token punctuation">,</span>
    <span class="token string">"full_name"</span><span class="token punctuation">:</span> <span class="token string">"Example User"</span><span class="token punctuation">,</span>
    <span class="token string">"has_password"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"2671355"</span><span class="token punctuation">,</span>
    <span class="token string">"image_id"</span><span class="token punctuation">:</span> <span class="token string">"d160009dfd52b991030d55227003450f"</span><span class="token punctuation">,</span>
    <span class="token string">"inbox_project_id"</span><span class="token punctuation">:</span> <span class="token string">"6X7fqH39MwjmwV4q"</span><span class="token punctuation">,</span>
    <span class="token string">"is_celebrations_enabled"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"is_premium"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"joinable_workspace"</span><span class="token punctuation">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
    <span class="token string">"joined_at"</span><span class="token punctuation">:</span> <span class="token string">"2015-07-31T18:32:06.000000Z"</span><span class="token punctuation">,</span>
    <span class="token string">"karma"</span><span class="token punctuation">:</span> <span class="token number">37504</span><span class="token punctuation">,</span>
    <span class="token string">"karma_trend"</span><span class="token punctuation">:</span> <span class="token string">"up"</span><span class="token punctuation">,</span>
    <span class="token string">"lang"</span><span class="token punctuation">:</span> <span class="token string">"en"</span><span class="token punctuation">,</span>
    <span class="token string">"mfa_enabled"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"next_week"</span><span class="token punctuation">:</span> <span class="token number">1</span><span class="token punctuation">,</span>
    <span class="token string">"premium_status"</span><span class="token punctuation">:</span> <span class="token string">"current_personal_plan"</span><span class="token punctuation">,</span>
    <span class="token string">"premium_until"</span><span class="token punctuation">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
    <span class="token string">"share_limit"</span><span class="token punctuation">:</span> <span class="token number">51</span><span class="token punctuation">,</span>
    <span class="token string">"sort_order"</span><span class="token punctuation">:</span> <span class="token number">0</span><span class="token punctuation">,</span>
    <span class="token string">"start_day"</span><span class="token punctuation">:</span> <span class="token number">1</span><span class="token punctuation">,</span>
    <span class="token string">"start_page"</span><span class="token punctuation">:</span> <span class="token string">"project?id=2203306141"</span><span class="token punctuation">,</span>
    <span class="token string">"theme_id"</span><span class="token punctuation">:</span> <span class="token string">"11"</span><span class="token punctuation">,</span>
    <span class="token string">"time_format"</span><span class="token punctuation">:</span> <span class="token number">0</span><span class="token punctuation">,</span>
    <span class="token string">"token"</span><span class="token punctuation">:</span> <span class="token string">"0123456789abcdef0123456789abcdef01234567"</span><span class="token punctuation">,</span>
    <span class="token string">"tz_info"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span>
        <span class="token string">"gmt_string"</span><span class="token punctuation">:</span> <span class="token string">"-03:00"</span><span class="token punctuation">,</span>
        <span class="token string">"hours"</span><span class="token punctuation">:</span> <span class="token operator">-</span><span class="token number">3</span><span class="token punctuation">,</span>
        <span class="token string">"is_dst"</span><span class="token punctuation">:</span> <span class="token number">0</span><span class="token punctuation">,</span>
        <span class="token string">"minutes"</span><span class="token punctuation">:</span> <span class="token number">0</span><span class="token punctuation">,</span>
        <span class="token string">"timezone"</span><span class="token punctuation">:</span> <span class="token string">"America/Sao_Paulo"</span>
    <span class="token punctuation">}</span><span class="token punctuation">,</span>
    <span class="token string">"verification_status"</span><span class="token punctuation">:</span> <span class="token string">"legacy"</span><span class="token punctuation">,</span>
    <span class="token string">"weekend_start_day"</span><span class="token punctuation">:</span> <span class="token number">6</span><span class="token punctuation">,</span>
    <span class="token string">"weekly_goal"</span><span class="token punctuation">:</span> <span class="token number">30</span>
<span class="token punctuation">}</span>
</code></pre>
<p>A Todoist user is represented by a JSON object. The dates will be in the UTC
timezone. Typically, a user object will have the following properties:</p>
<h4 id="properties">Properties</h4>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>auto_reminder <em>Integer</em></td>
<td>The default time in minutes for the automatic reminders set, whenever a due date has been specified for a task.</td>
</tr>
<tr>
<td>avatar_big <em>String</em></td>
<td>The link to a 195x195 pixels image of the user&#39;s avatar.</td>
</tr>
<tr>
<td>avatar_medium <em>String</em></td>
<td>The link to a 60x60 pixels image of the user&#39;s avatar.</td>
</tr>
<tr>
<td>avatar_s640 <em>String</em></td>
<td>The link to a 640x640 pixels image of the user&#39;s avatar.</td>
</tr>
<tr>
<td>avatar_small <em>String</em></td>
<td>The link to a 35x35 pixels image of the user&#39;s avatar.</td>
</tr>
<tr>
<td>business_account_id <em>String</em></td>
<td>The ID of the user&#39;s business account.</td>
</tr>
<tr>
<td>daily_goal <em>Integer</em></td>
<td>The daily goal number of completed tasks for karma.</td>
</tr>
<tr>
<td>date_format <em>Integer</em></td>
<td>Whether to use the <code>DD-MM-YYYY</code> date format (if set to <code>0</code>), or the <code>MM-DD-YYYY</code> format (if set to <code>1</code>).</td>
</tr>
<tr>
<td>dateist_lang <em>String</em></td>
<td>The language expected for date recognition instead of the user&#39;s <code>lang</code> (<code>null</code> if the user&#39;s <code>lang</code> determines this), one of the following values: <code>da</code>, <code>de</code>, <code>en</code>, <code>es</code>, <code>fi</code>, <code>fr</code>, <code>it</code>, <code>ja</code>, <code>ko</code>, <code>nl</code>, <code>pl</code>, <code>pt_BR</code>, <code>ru</code>, <code>sv</code>, <code>tr</code>, <code>zh_CN</code>, <code>zh_TW</code>.</td>
</tr>
<tr>
<td>days_off <em>Array</em></td>
<td>Array of integers representing user&#39;s days off (between <code>1</code> and <code>7</code>, where <code>1</code> is <code>Monday</code> and <code>7</code> is <code>Sunday</code>).</td>
</tr>
<tr>
<td>email <em>String</em></td>
<td>The user&#39;s email.</td>
</tr>
<tr>
<td>feature_identifier <em>String</em></td>
<td>An opaque id used internally to handle features for the user.</td>
</tr>
<tr>
<td>features <em>Object</em></td>
<td>Used internally for any special features that apply to the user. Current special features include whether the user has enabled <code>beta</code>, whether <code>dateist_inline_disabled</code> that is inline date parsing support is disabled, whether the <code>dateist_lang</code> is set which overrides the date parsing language, whether the <code>gold_theme</code> has been awarded to the user, whether the user <code>has_push_reminders</code> enabled, whether the user has <code>karma_disabled</code>, whether the user has <code>karma_vacation</code> mode enabled, and whether any special <code>restriction</code> applies to the user.</td>
</tr>
<tr>
<td>full_name <em>String</em></td>
<td>The user&#39;s real name formatted as <code>Firstname Lastname</code>.</td>
</tr>
<tr>
<td>has_password <em>Boolean</em></td>
<td>Whether the user has a password set on the account. It will be <code>false</code> if they have only authenticated without a password (e.g. using Google, Facebook, etc.)</td>
</tr>
<tr>
<td>id <em>String</em></td>
<td>The user&#39;s ID.</td>
</tr>
<tr>
<td>image_id <em>String</em></td>
<td>The ID of the user&#39;s avatar.</td>
</tr>
<tr>
<td>inbox_project_id <em>String</em></td>
<td>The ID of the user&#39;s <code>Inbox</code> project.</td>
</tr>
<tr>
<td>is_premium <em>Boolean</em></td>
<td>Whether the user has a Todoist Pro subscription (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>joined_at <em>String</em></td>
<td>The date when the user joined Todoist.</td>
</tr>
<tr>
<td>karma <em>Integer</em></td>
<td>The user&#39;s karma score.</td>
</tr>
<tr>
<td>karma_trend <em>String</em></td>
<td>The user&#39;s karma trend (for example <code>up</code>).</td>
</tr>
<tr>
<td>lang <em>String</em></td>
<td>The user&#39;s language, which can take one of the following values: <code>da</code>, <code>de</code>, <code>en</code>, <code>es</code>, <code>fi</code>, <code>fr</code>, <code>it</code>, <code>ja</code>, <code>ko</code>, <code>nl</code>, <code>pl</code>, <code>pt_BR</code>, <code>ru</code>, <code>sv</code>, <code>tr</code>, <code>zh_CN</code>, <code>zh_TW</code>.</td>
</tr>
<tr>
<td>next_week <em>Integer</em></td>
<td>The day of the next week, that tasks will be postponed to (between <code>1</code> and <code>7</code>, where <code>1</code> is <code>Monday</code> and <code>7</code> is <code>Sunday</code>).</td>
</tr>
<tr>
<td>premium_status <em>String</em></td>
<td>Outlines why a user is premium, possible values are: <code>not_premium</code>, <code>current_personal_plan</code>, <code>legacy_personal_plan</code> or <code>teams_business_member</code>.</td>
</tr>
<tr>
<td>premium_until <em>String</em></td>
<td>The date when the user&#39;s Todoist Pro subscription ends (<code>null</code> if not a Todoist Pro user). This should be used for informational purposes only as this does not include the grace period upon expiration. As a result, avoid using this to determine whether someone has a Todoist Pro subscription and use <code>is_premium</code> instead.</td>
</tr>
<tr>
<td>sort_order <em>Integer</em></td>
<td>Whether to show projects in an <code>oldest dates first</code> order (if set to <code>0</code>, or a <code>oldest dates last</code> order (if set to <code>1</code>).</td>
</tr>
<tr>
<td>start_day <em>Integer</em></td>
<td>The first day of the week (between <code>1</code> and <code>7</code>, where <code>1</code> is <code>Monday</code> and <code>7</code> is <code>Sunday</code>).</td>
</tr>
<tr>
<td>start_page <em>String</em></td>
<td>The user&#39;s default view on Todoist. The start page can be one of the following: <code>inbox</code>, <code>teaminbox</code>, <code>today</code>, <code>next7days</code>, <code>project?id=1234</code> to open a project, <code>label?name=abc</code> to open a label, <code>filter?id=1234</code> to open a personal filter or <code>workspace_filter?id=1234</code> to open a workspace filter.</td>
</tr>
<tr>
<td>theme_id <em>String</em></td>
<td>The currently selected Todoist theme (a number between <code>0</code> and <code>10</code>).</td>
</tr>
<tr>
<td>time_format <em>Integer</em></td>
<td>Whether to use a <code>24h</code> format such as <code>13:00</code> (if set to <code>0</code>) when displaying time, or a <code>12h</code> format such as <code>1:00pm</code> (if set to <code>1</code>).</td>
</tr>
<tr>
<td>token <em>String</em></td>
<td>The user&#39;s token that should be used to call the other API methods.</td>
</tr>
<tr>
<td>tz_info <em>Object</em></td>
<td>The user&#39;s timezone (a dictionary structure), which includes the following elements: the <code>timezone</code> as a string value, the <code>hours</code> and <code>minutes</code> difference from GMT, whether daylight saving time applies denoted by <code>is_dst</code>, and a string value of the time difference from GMT that is <code>gmt_string</code>.</td>
</tr>
<tr>
<td>weekend_start_day <em>Integer</em></td>
<td>The day used when a user chooses to schedule a task for the &#39;Weekend&#39; (between 1 and 7, where 1 is Monday and 7 is Sunday).</td>
</tr>
<tr>
<td>verification_status <em>String</em></td>
<td>Describes if the user has verified their e-mail address or not. Possible values are:</td>
</tr>
</tbody></table>
<ul>
<li><code>unverified</code>, for users that have just signed up. Those users cannot use any of Todoist&#39;s social features like sharing projects or accepting project invitations.</li>
<li><code>verified</code>, for users that have verified themselves somehow. Clicking on the verification link inside the account confirmation e-mail is one such way alongside signing up through a social account.</li>
<li><code>blocked</code>, for users that have failed to verify themselves in 7 days. Those users will have restricted usage of Todoist.</li>
<li><code>legacy</code>, for users that have signed up before August, 2022 weekly_goal <em>Integer</em> | The target number of tasks to complete per week.</li>
</ul>

<blockquote>
<p>Example update user request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "user_update",
        "uuid": "52f83009-7e27-4b9f-9943-1c5e3d1e6889",
        "args": {
            "current_password": "fke4iorij",
            "email": "mynewemail@example.com"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"52f83009-7e27-4b9f-9943-1c5e3d1e6889"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>current_password <em>String</em></td>
<td>Yes (if modifying <code>email</code> or <code>password</code>)</td>
<td>The user&#39;s current password. This must be provided if the request is modifying the user&#39;s password or email address and the user already has a password set (indicated by <code>has_password</code> in the <a href="#tag/Sync/User">user</a> object). For amending other properties this is not required.</td>
</tr>
<tr>
<td>email <em>String</em></td>
<td>No</td>
<td>The user&#39;s email.</td>
</tr>
<tr>
<td>full_name <em>String</em></td>
<td>No</td>
<td>The user&#39;s name.</td>
</tr>
<tr>
<td>password <em>String</em></td>
<td>No</td>
<td>The user&#39;s updated password. Must contain at least 8 characters and not be a common or easily guessed password.</td>
</tr>
<tr>
<td>timezone <em>String</em></td>
<td>No</td>
<td>The user&#39;s timezone (a string value such as <code>UTC</code>, <code>Europe/Lisbon</code>, <code>US/Eastern</code>, <code>Asia/Taipei</code>).</td>
</tr>
<tr>
<td>start_page <em>String</em></td>
<td>No</td>
<td>The user&#39;s default view on Todoist. The start page can be one of the following: <code>inbox</code>, <code>teaminbox</code>, <code>today</code>, <code>next7days</code>, <code>project?id=1234</code> to open a project, <code>label?name=abc</code> to open a label, <code>filter?id=1234</code> to open a personal filter or <code>workspace_filter?id=1234</code> to open a workspace filter.</td>
</tr>
<tr>
<td>start_day <em>Integer</em></td>
<td>No</td>
<td>The first day of the week (between <code>1</code> and <code>7</code>, where <code>1</code> is <code>Monday</code> and <code>7</code> is <code>Sunday</code>).</td>
</tr>
<tr>
<td>next_week <em>Integer</em></td>
<td>No</td>
<td>The day of the next week, that tasks will be postponed to (between <code>1</code> and <code>7</code>, where <code>1</code> is <code>Monday</code> and <code>7</code> is <code>Sunday</code>).</td>
</tr>
<tr>
<td>time_format <em>Integer</em></td>
<td>No</td>
<td>Whether to use a <code>24h</code> format such as <code>13:00</code> (if set to <code>0</code>) when displaying time, or a <code>12h</code> format such as <code>1:00pm</code> (if set to <code>1</code>).</td>
</tr>
<tr>
<td>date_format <em>Integer</em></td>
<td>No</td>
<td>Whether to use the <code>DD-MM-YYYY</code> date format (if set to <code>0</code>), or the <code>MM-DD-YYYY</code> format (if set to <code>1</code>).</td>
</tr>
<tr>
<td>sort_order <em>Integer</em></td>
<td>No</td>
<td>Whether to show projects in an <code>oldest dates first</code> order (if set to <code>0</code>, or a <code>oldest dates last</code> order (if set to <code>1</code>).</td>
</tr>
<tr>
<td>auto_reminder <em>Integer</em></td>
<td>No</td>
<td>The default time in minutes for the automatic reminders set, whenever a due date has been specified for a task.</td>
</tr>
<tr>
<td>theme <em>Integer</em></td>
<td>No</td>
<td>The currently selected Todoist theme (between <code>0</code> and <code>10</code>).</td>
</tr>
<tr>
<td>weekend_start_day <em>Integer</em></td>
<td>No</td>
<td>The day used when a user chooses to schedule a task for the &#39;Weekend&#39; (between 1 and 7, where 1 is Monday and 7 is Sunday).</td>
</tr>
<tr>
<td>beta <em>Boolean</em></td>
<td>No</td>
<td>Whether the user is included in the beta testing group.</td>
</tr>
<tr>
<td>onboarding_completed <em>Boolean</em></td>
<td>No</td>
<td>For first-party clients usage only. This attribute may be removed or changed without notice, so we strongly advise not to rely on it.</td>
</tr>
<tr>
<td>onboarding_initiated <em>Boolean</em></td>
<td>No</td>
<td>For first-party clients usage only. This attribute may be removed or changed without notice, so we strongly advise not to rely on it.</td>
</tr>
<tr>
<td>onboarding_level <em>String</em></td>
<td>No</td>
<td>For first-party clients usage only. The onboarding level (<code>pro</code>, <code>intermediate</code>, <code>beginner</code>). This attribute may be removed or changed without notice, so we strongly advise not to rely on it.</td>
</tr>
<tr>
<td>onboarding_persona <em>String</em></td>
<td>No</td>
<td>For first-party clients usage only. The onboarding persona (<code>analog</code>, <code>tasks</code>, <code>calendar</code>, <code>organic</code>). This attribute may be removed or changed without notice, so we strongly advise not to rely on it.</td>
</tr>
<tr>
<td>onboarding_role <em>String</em></td>
<td>No</td>
<td>For first-party clients usage only. The onboarding role (<code>leader</code>, <code>founder</code>, <code>ic</code>). This attribute may be removed or changed without notice, so we strongly advise not to rely on it.</td>
</tr>
<tr>
<td>onboarding_skipped <em>Boolean</em></td>
<td>No</td>
<td>For first-party clients usage only. This attribute may be removed or changed without notice, so we strongly advise not to rely on it.</td>
</tr>
<tr>
<td>onboarding_started <em>Boolean</em></td>
<td>No</td>
<td>For first-party clients usage only. This attribute may be removed or changed without notice, so we strongly advise not to rely on it.</td>
</tr>
<tr>
<td>onboarding_team_mode <em>Boolean</em></td>
<td>No</td>
<td>For first-party clients usage only. This attribute may be removed or changed without notice, so we strongly advise not to rely on it.</td>
</tr>
<tr>
<td>onboarding_use_cases <em>Array</em></td>
<td>No</td>
<td>For first-party clients usage only. JSON array of onboarding use cases (<code>personal</code>, <code>work</code>, <code>education</code>, <code>teamwork</code>, <code>solo</code>, <code>teamcreator</code>, <code>simple</code>, <code>teamjoiner</code>). This attribute may be removed or changed without notice, so we strongly advise not to rely on it.</td>
</tr>
<tr>
<td>completed_guide_project_id <em>String</em></td>
<td>No</td>
<td>For first-party clients usage only. Mark a Getting Started Guide project as completed by providing its project ID. This attribute may be removed or changed without notice, so we strongly advise not to rely on it.</td>
</tr>
<tr>
<td>closed_guide_project_id <em>String</em></td>
<td>No</td>
<td>For first-party clients usage only. Mark a Getting Started Guide project as closed (dismissed) by providing its project ID. This attribute may be removed or changed without notice, so we strongly advise not to rely on it.</td>
</tr>
<tr>
<td>getting_started_guide_projects <em>String</em></td>
<td>No</td>
<td>For first-party clients usage only. JSON array of Getting Started guide projects with completion tracking. Each project contains <code>project_id</code>, <code>onboarding_use_case</code>, <code>completed</code>, and <code>closed</code> status. This attribute may be removed or changed without notice, so we strongly advise not to rely on it.</td>
</tr>
</tbody></table>
<h4 id="error-codes">Error codes</h4>
<table>
<thead>
<tr>
<th>Error Tag</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td><code>PASSWORD_REQUIRED</code></td>
<td>The command attempted to modify <code>password</code> or <code>email</code>, but no value was provided for <code>current_password</code>.</td>
</tr>
<tr>
<td><code>AUTHENTICATION_ERROR</code></td>
<td>The value for <code>current_password</code> was incorrect.</td>
</tr>
<tr>
<td><code>PASSWORD_TOO_SHORT</code></td>
<td>The value for <code>password</code> was shorter than the minimum 8 characters.</td>
</tr>
<tr>
<td><code>COMMON_PASSWORD</code></td>
<td>The value for <code>password</code> was matched against a common password list and rejected.</td>
</tr>
<tr>
<td><code>PASSWORD_CONTAINS_EMAIL</code></td>
<td>The value for password was matched against the user&#39;s email address or a part of the address.</td>
</tr>
<tr>
<td><code>INVALID_EMAIL</code></td>
<td>The value for <code>email</code> was not a valid email address.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example update karma goals request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "update_goals",
        "uuid": "b9bbeaf8-9db6-452a-a843-a192f1542892",
        "args": {"vacation_mode": 1}
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"b9bbeaf8-9db6-452a-a843-a192f1542892"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Update the karma goals of the user.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>daily_goal <em>Integer</em></td>
<td>No</td>
<td>The target number of tasks to complete per day.</td>
</tr>
<tr>
<td>weekly_goal <em>Integer</em></td>
<td>No</td>
<td>The target number of tasks to complete per week.</td>
</tr>
<tr>
<td>ignore_days <em>Integer</em></td>
<td>No</td>
<td>A list with the days of the week to ignore (<code>1</code> for <code>Monday</code> and <code>7</code> for <code>Sunday</code>).</td>
</tr>
<tr>
<td>vacation_mode <em>Integer</em></td>
<td>No</td>
<td>Marks the user as being on vacation (where <code>1</code> is true and <code>0</code> is false).</td>
</tr>
<tr>
<td>karma_disabled <em>Integer</em></td>
<td>No</td>
<td>Whether to disable the karma and goals measuring altogether (where <code>1</code> is true and <code>0</code> is false).</td>
</tr>
</tbody></table>

<blockquote>
<p>An example user plan limits sync response</p>
</blockquote>
<pre><code><span class="token punctuation">{</span>
    <span class="token string">"user_plan_limits"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span>
        <span class="token string">"current"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span>
            <span class="token string">"plan_name"</span><span class="token punctuation">:</span> <span class="token string">"free"</span><span class="token punctuation">,</span>
            <span class="token punctuation">.</span><span class="token punctuation">.</span><span class="token punctuation">.</span>details of the current user plan
        <span class="token punctuation">}</span><span class="token punctuation">,</span>
        <span class="token string">"next"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span>
            <span class="token string">"plan_name"</span><span class="token punctuation">:</span> <span class="token string">"pro"</span><span class="token punctuation">,</span>
            <span class="token punctuation">.</span><span class="token punctuation">.</span><span class="token punctuation">.</span>details of a potential upgrade
        <span class="token punctuation">}</span>
    <span class="token punctuation">}</span>
<span class="token punctuation">}</span>
</code></pre>
<p>The <code>user_plan_limits</code> sync resource type describes the available features and
limits applicable to the current user plan. The user plan info object (detailed
in the next section) returned within the <code>current</code> property shows the values
that are currently applied to the user.</p>
<p>If there is an upgrade available, the <code>next</code> property will show the values that will apply if the user chooses
to upgrade. If there is no available upgrade, the <code>next</code> value will be null.</p>
<h4 id="properties">Properties</h4>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>current <em>Object</em></td>
<td>A user plan info object representing the available functionality and limits for the user&#39;s current plan.</td>
</tr>
<tr>
<td>next <em>Object</em></td>
<td>A user plan info object representing the plan available for upgrade. If there is no available upgrade, this value will be null.</td>
</tr>
</tbody></table>
<h3 id="user-plan-info">User plan info</h3>
<blockquote>
<p>An example user plan info object</p>
</blockquote>
<pre><code><span class="token punctuation">{</span>
    <span class="token string">"activity_log"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"activity_log_limit"</span><span class="token punctuation">:</span> <span class="token number">7</span><span class="token punctuation">,</span>
    <span class="token string">"advanced_permissions"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"automatic_backups"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"calendar_feeds"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"calendar_layout"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"comments"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"completed_tasks"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"custom_app_icon"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"customization_color"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"deadlines"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"durations"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"email_forwarding"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"filters"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"labels"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"max_calendar_accounts"</span><span class="token punctuation">:</span> <span class="token number">1</span><span class="token punctuation">,</span>
    <span class="token string">"max_collaborators"</span><span class="token punctuation">:</span> <span class="token number">5</span><span class="token punctuation">,</span>
    <span class="token string">"max_filters"</span><span class="token punctuation">:</span> <span class="token number">3</span><span class="token punctuation">,</span>
    <span class="token string">"max_folders_per_workspace"</span><span class="token punctuation">:</span> <span class="token number">25</span><span class="token punctuation">,</span>
    <span class="token string">"max_workspace_filters"</span><span class="token punctuation">:</span> <span class="token number">3</span><span class="token punctuation">,</span>
    <span class="token string">"workspace_filters"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"max_free_workspaces_created"</span><span class="token punctuation">:</span> <span class="token number">1</span><span class="token punctuation">,</span>
    <span class="token string">"max_guests_per_workspace"</span><span class="token punctuation">:</span> <span class="token number">25</span><span class="token punctuation">,</span>
    <span class="token string">"max_labels"</span><span class="token punctuation">:</span> <span class="token number">500</span><span class="token punctuation">,</span>
    <span class="token string">"max_projects"</span><span class="token punctuation">:</span> <span class="token number">5</span><span class="token punctuation">,</span>
    <span class="token string">"max_projects_joined"</span><span class="token punctuation">:</span> <span class="token number">500</span><span class="token punctuation">,</span>
    <span class="token string">"max_reminders_location"</span><span class="token punctuation">:</span> <span class="token number">300</span><span class="token punctuation">,</span>
    <span class="token string">"max_reminders_time"</span><span class="token punctuation">:</span> <span class="token number">700</span><span class="token punctuation">,</span>
    <span class="token string">"max_sections"</span><span class="token punctuation">:</span> <span class="token number">20</span><span class="token punctuation">,</span>
    <span class="token string">"max_tasks"</span><span class="token punctuation">:</span> <span class="token number">300</span><span class="token punctuation">,</span>
    <span class="token string">"max_user_templates"</span><span class="token punctuation">:</span> <span class="token number">100</span><span class="token punctuation">,</span>
    <span class="token string">"plan_name"</span><span class="token punctuation">:</span> <span class="token string">"free"</span><span class="token punctuation">,</span>
    <span class="token string">"reminders"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"reminders_at_due"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"templates"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"upload_limit_mb"</span><span class="token punctuation">:</span> <span class="token number">5</span><span class="token punctuation">,</span>
    <span class="token string">"uploads"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"weekly_trends"</span><span class="token punctuation">:</span> <span class="token boolean">true</span>
<span class="token punctuation">}</span>
</code></pre>
<p>The user plan info object describes the availability of features and any limitations applied for a given user plan.</p>
<h4 id="properties-1">Properties</h4>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>plan_name <em>String</em></td>
<td>The name of the plan.</td>
</tr>
<tr>
<td>activity_log <em>Boolean</em></td>
<td>Whether the user can view the <a href="#tag/Activity">activity log</a>.</td>
</tr>
<tr>
<td>activity_log_limit <em>Integer</em></td>
<td>The number of days of history that will be displayed within the activity log. If there is no limit, the value will be <code>-1</code>.</td>
</tr>
<tr>
<td>automatic_backups <em>Boolean</em></td>
<td>Whether <a href="#tag/Backups">backups</a> will be automatically created for the user&#39;s account and available for download.</td>
</tr>
<tr>
<td>calendar_feeds <em>Boolean</em></td>
<td>Whether calendar feeds can be enabled for the user&#39;s projects.</td>
</tr>
<tr>
<td>comments <em>Boolean</em></td>
<td>Whether the user can add <a href="#tag/Sync/Comments">comments</a>.</td>
</tr>
<tr>
<td>completed_tasks <em>Boolean</em></td>
<td>Whether the user can search in the completed tasks archive or access the completed tasks overview.</td>
</tr>
<tr>
<td>custom_app_icon <em>Boolean</em></td>
<td>Whether the user can set a custom app icon on the iOS app.</td>
</tr>
<tr>
<td>customization_color <em>Boolean</em></td>
<td>Whether the user can use special themes or other visual customization.</td>
</tr>
<tr>
<td>email_forwarding <em>Boolean</em></td>
<td>Whether the user can add tasks or comments via <a href="#tag/Emails">email</a>.</td>
</tr>
<tr>
<td>filters <em>Boolean</em></td>
<td>Whether the user can add and update <a href="#tag/Sync/Filters">filters</a>.</td>
</tr>
<tr>
<td>max_filters <em>Integer</em></td>
<td>The maximum number of filters a user can have.</td>
</tr>
<tr>
<td>workspace_filters <em>Boolean</em></td>
<td>Whether the user can add and update <a href="#tag/Sync/Workspace-Filters">workspace filters</a> (Business/Enterprise plans only).</td>
</tr>
<tr>
<td>max_workspace_filters <em>Integer</em></td>
<td>The maximum number of workspace filters a user can have per workspace.</td>
</tr>
<tr>
<td>labels <em>Boolean</em></td>
<td>Whether the user can add <a href="#tag/Sync/Labels">labels</a>.</td>
</tr>
<tr>
<td>max_labels <em>Integer</em></td>
<td>The maximum number of labels a user can have.</td>
</tr>
<tr>
<td>reminders <em>Boolean</em></td>
<td>Whether the user can add <a href="#tag/Sync/Reminders">reminders</a>.</td>
</tr>
<tr>
<td>max_reminders_location <em>Integer</em></td>
<td>The maximum number of location reminders a user can have.</td>
</tr>
<tr>
<td>max_reminders_time <em>Integer</em></td>
<td>The maximum number of time-based reminders a user can have.</td>
</tr>
<tr>
<td>templates <em>Boolean</em></td>
<td>Whether the user can import and export <a href="#tag/Templates">project templates</a>.</td>
</tr>
<tr>
<td>uploads <em>Boolean</em></td>
<td>Whether the user can <a href="#tag/Uploads">upload attachments</a>.</td>
</tr>
<tr>
<td>upload_limit_mb <em>Integer</em></td>
<td>The maximum size of an individual file the user can upload.</td>
</tr>
<tr>
<td>weekly_trends <em>Boolean</em></td>
<td>Whether the user can view <a href="#tag/Sync/User">productivity stats</a>.</td>
</tr>
<tr>
<td>max_projects <em>Integer</em></td>
<td>The maximum number of active <a href="#tag/Sync/Projects">projects</a> a user can have.</td>
</tr>
<tr>
<td>max_sections <em>Integer</em></td>
<td>The maximum number of active <a href="#tag/Sync/Sections">sections</a> a user can have.</td>
</tr>
<tr>
<td>max_tasks <em>Integer</em></td>
<td>The maximum number of active <a href="#tag/Sync/Tasks">tasks</a> a user can have.</td>
</tr>
<tr>
<td>max_collaborators <em>Integer</em></td>
<td>The maximum number of <a href="#tag/Sync/Sharing/Collaborators">collaborators</a> a user can add to a project.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example user settings object:</p>
</blockquote>
<pre><code><span class="token punctuation">{</span>
    <span class="token string">"reminder_push"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"reminder_desktop"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"reminder_email"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"completed_sound_desktop"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"completed_sound_mobile"</span><span class="token punctuation">:</span> <span class="token boolean">true</span>
<span class="token punctuation">}</span>
</code></pre>
<p><em>Availability of reminders functionality is dependent on the current user plan.
This value is indicated by the <code>reminders</code> property of the <a href="#tag/Sync/User/User-plan-limits">user plan limits</a> object.
These settings will have no effect if the user is not eligible for reminders.</em></p>
<h4 id="properties">Properties</h4>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>reminder_push <em>Boolean</em></td>
<td>Set to true to send reminders as push notifications.</td>
</tr>
<tr>
<td>reminder_desktop <em>Boolean</em></td>
<td>Set to true to show reminders in desktop applications.</td>
</tr>
<tr>
<td>reminder_email <em>Boolean</em></td>
<td>Set to true to send reminders by email.</td>
</tr>
<tr>
<td>completed_sound_desktop <em>Boolean</em></td>
<td>Set to true to enable sound when a task is completed in Todoist desktop clients.</td>
</tr>
<tr>
<td>completed_sound_mobile <em>Boolean</em></td>
<td>Set to true to enable sound when a task is completed in Todoist mobile clients.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example stats object:</p>
</blockquote>
<pre><code class="language-json"><span class="token punctuation">{</span>
  <span class="token string-property property">"completed_count"</span><span class="token operator">:</span> <span class="token number">123</span><span class="token punctuation">,</span>
  <span class="token string-property property">"days_items"</span><span class="token operator">:</span> <span class="token punctuation">[</span>
    <span class="token punctuation">{</span>
      <span class="token string-property property">"date"</span><span class="token operator">:</span> <span class="token string">"2025-10-17"</span><span class="token punctuation">,</span>
      <span class="token string-property property">"total_completed"</span><span class="token operator">:</span> <span class="token number">5</span>
    <span class="token punctuation">}</span>
  <span class="token punctuation">]</span><span class="token punctuation">,</span>
  <span class="token string-property property">"week_items"</span><span class="token operator">:</span> <span class="token punctuation">[</span>
    <span class="token punctuation">{</span>
      <span class="token string-property property">"from"</span><span class="token operator">:</span> <span class="token string">"2025-10-13"</span><span class="token punctuation">,</span>
      <span class="token string-property property">"to"</span><span class="token operator">:</span> <span class="token string">"2025-10-19"</span><span class="token punctuation">,</span>
      <span class="token string-property property">"total_completed"</span><span class="token operator">:</span> <span class="token number">12</span>
    <span class="token punctuation">}</span>
  <span class="token punctuation">]</span>
<span class="token punctuation">}</span>
</code></pre>
<h4 id="properties">Properties</h4>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>completed_count <em>Integer</em></td>
<td>The total number of tasks the user has completed across all time.</td>
</tr>
<tr>
<td>days_items <em>Array</em></td>
<td>An array containing completion statistics for today. Each item contains <code>date</code> and <code>total_completed</code>.</td>
</tr>
<tr>
<td>week_items <em>Array</em></td>
<td>An array containing completion statistics for the current week. Each item contains <code>from</code>, <code>to</code>, and <code>total_completed</code>.</td>
</tr>
</tbody></table>
<h3 id="update-user-settings">Update user settings</h3>
<blockquote>
<p>Example update user settings request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "user_settings_update",
        "temp_id": "e24ad822-a0df-4b7d-840f-83a5424a484a",
        "uuid": "41e59a76-3430-4e44-92b9-09d114be0d49",
        "args": {"reminder_desktop": false}
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"41e59a76-3430-4e44-92b9-09d114be0d49"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Update one or more user settings.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>reminder_push <em>Boolean</em></td>
<td>No</td>
<td>Set to true to send reminders as push notifications.</td>
</tr>
<tr>
<td>reminder_desktop <em>Boolean</em></td>
<td>No</td>
<td>Set to true to show reminders in desktop applications.</td>
</tr>
<tr>
<td>reminder_email <em>Boolean</em></td>
<td>No</td>
<td>Set to true to send reminders by email.</td>
</tr>
<tr>
<td>completed_sound_desktop <em>Boolean</em></td>
<td>No</td>
<td>Set to true to enable sound when a task is completed in Todoist desktop clients.</td>
</tr>
<tr>
<td>completed_sound_mobile <em>Boolean</em></td>
<td>No</td>
<td>Set to true to enable sound when a task is completed in Todoist mobile clients.</td>
</tr>
</tbody></table>

<p>Projects can be shared with other users, which are then referred to as collaborators.
This section describes the different commands that are related to sharing.</p>

<blockquote>
<p>An example collaborator object:</p>
</blockquote>
<pre><code><span class="token punctuation">{</span>
    <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"2671362"</span><span class="token punctuation">,</span>
    <span class="token string">"email"</span><span class="token punctuation">:</span> <span class="token string">"you@example.com"</span><span class="token punctuation">,</span>
    <span class="token string">"full_name"</span><span class="token punctuation">:</span> <span class="token string">"Example User"</span><span class="token punctuation">,</span>
    <span class="token string">"timezone"</span><span class="token punctuation">:</span> <span class="token string">"GMT +3:00"</span><span class="token punctuation">,</span>
    <span class="token string">"image_id"</span><span class="token punctuation">:</span> <span class="token keyword">null</span>
<span class="token punctuation">}</span>
</code></pre>
<p>There are two types of objects to get information about a user’s collaborators,
and their participation in shared projects: <code>collaborators</code> and
<code>collaborator_states</code></p>
<p>Every user who shares at least one project with another user, has a
collaborators record in the API response. The record contains a restricted
subset of user-specific properties.</p>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>The user ID of the collaborator.</td>
</tr>
<tr>
<td>email <em>String</em></td>
<td>The email of the collaborator.</td>
</tr>
<tr>
<td>full_name <em>String</em></td>
<td>The full name of the collaborator.</td>
</tr>
<tr>
<td>timezone <em>String</em></td>
<td>The timezone of the collaborator.</td>
</tr>
<tr>
<td>image_id <em>String</em></td>
<td>The image ID for the collaborator&#39;s avatar, which can be used to get an avatar from a specific URL. Specifically the <code>https://dcff1xvirvpfp.cloudfront.net/&lt;image_id&gt;_big.jpg</code> can be used for a big (<code>195x195</code> pixels) avatar, <code>https://dcff1xvirvpfp.cloudfront.net/&lt;image_id&gt;_medium.jpg</code> for a medium (<code>60x60</code> pixels) avatar, and <code>https://dcff1xvirvpfp.cloudfront.net/&lt;image_id&gt;_small.jpg</code> for a small (<code>35x35</code> pixels) avatar.</td>
</tr>
</tbody></table>
<p>Partial sync returns updated collaborator objects for users that have changed
their attributes, such as their name or email.</p>

<blockquote>
<p>An example collaborator state:</p>
</blockquote>
<pre><code><span class="token punctuation">{</span>
    <span class="token string">"project_id"</span><span class="token punctuation">:</span> <span class="token string">"6H2c63wj7x9hFJfX"</span><span class="token punctuation">,</span>
    <span class="token string">"user_id"</span><span class="token punctuation">:</span> <span class="token string">"2671362"</span><span class="token punctuation">,</span>
    <span class="token string">"state"</span><span class="token punctuation">:</span> <span class="token string">"active"</span><span class="token punctuation">,</span>
    <span class="token string">"is_deleted"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"role"</span><span class="token punctuation">:</span> <span class="token string">"READ_WRITE"</span><span class="token punctuation">,</span>
<span class="token punctuation">}</span>
</code></pre>
<p>The list of collaborators don’t contain any information on how users are
connected to shared projects. To provide information about these connections,
the <code>collaborator_states</code> field should be used. Every collaborator state record
is a mere &quot;user to shared project&quot; mapping.</p>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>project_id <em>String</em></td>
<td>The shared project ID of the user.</td>
</tr>
<tr>
<td>user_id <em>String</em></td>
<td>The user ID of the collaborator.</td>
</tr>
<tr>
<td>state <em>String</em></td>
<td>The status of the collaborator state, either <code>active</code> or <code>invited</code>.</td>
</tr>
<tr>
<td>is_deleted <em>Boolean</em></td>
<td>Set to <code>true</code> when the collaborator leaves the shared project.</td>
</tr>
<tr>
<td>role</td>
<td>The role of the collaborator in the project. <em>Only available for teams</em></td>
</tr>
</tbody></table>

<blockquote>
<p>Example share project request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "share_project",
        "temp_id": "854be9cd-965f-4ddd-a07e-6a1d4a6e6f7a",
        "uuid": "fe6637e3-03ce-4236-a202-8b28de2c8372",
        "args": {
            "project_id": "6H2c63wj7x9hFJfX",
            "email": "you@example.com"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"fe6637e3-03ce-4236-a202-8b28de2c8372"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Share a project with another user.</p>
<p><em>When sharing a teams project</em></p>
<p>Workspace projects with <code>is_invite_only</code> set to true can only be shared by
workspace admins, or by project members with <code>ADMIN</code> or <code>CREATOR</code> role. Other
users will get a “forbidden” error. The role for the new collaborator cannot be
greater than the role of the person sharing the project.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>project_id <em>String</em></td>
<td>Yes</td>
<td>The project to be shared.</td>
</tr>
<tr>
<td>email <em>String</em></td>
<td>Yes</td>
<td>The user email with whom to share the project.</td>
</tr>
<tr>
<td>role <em>String</em></td>
<td>No</td>
<td>The role of the new collaborator in the workspace project. If missing, the workspace <code>collaborator_role_default</code> will be used. <em>Only used for teams</em></td>
</tr>
</tbody></table>

<blockquote>
<p>Example delete collaborator request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "delete_collaborator",
        "uuid": "0ae55ac0-3b8d-4835-b7c3-59ba30e73ae4",
        "args": {
            "project_id": "6H2c63wj7x9hFJfX",
            "email": "you@example.com"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"0ae55ac0-3b8d-4835-b7c3-59ba30e73ae4"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Remove a user from a shared project.
In Teams, only workspace admins or project members with <code>ADMIN</code> or <code>CREATOR</code> role can delete a collaborator. Other users will get a “forbidden” error.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>project_id <em>String</em></td>
<td>Yes</td>
<td>The project to be affected.</td>
</tr>
<tr>
<td>email <em>String</em></td>
<td>Yes</td>
<td>The user email with whom the project was shared.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example accept invitation request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "accept_invitation",
        "uuid": "4b254da4-fa2b-4a88-9439-b27903a90f7f",
        "args": {
            "invitation_id": "1234",
            "invitation_secret": "abcdefghijklmno"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"4b254da4-fa2b-4a88-9439-b27903a90f7f"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Accept an invitation to join a shared project.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>invitation_id <em>String</em></td>
<td>Yes</td>
<td>The invitation ID.</td>
</tr>
<tr>
<td>invitation_secret <em>String</em></td>
<td>Yes</td>
<td>The secret fetched from the live notification.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example reject invitation request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "reject_invitation",
        "uuid": "284fd900-c36f-44e5-ab92-ee93455e50e0",
        "args": {
            "invitation_id": "1234",
            "invitation_secret": "abcdefghijklmno"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"284fd900-c36f-44e5-ab92-ee93455e50e0"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Reject an invitation to join a shared project.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>invitation_id <em>String</em></td>
<td>Yes</td>
<td>The invitation ID.</td>
</tr>
<tr>
<td>invitation_secret <em>String</em></td>
<td>Yes</td>
<td>The secret fetched from the live notification.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example delete invitation request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "delete_invitation",
        "uuid": "399f6a8d-ddea-4146-ae8e-b41fb8ff6945",
        "args": {"invitation_id": "1234"}
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"399f6a8d-ddea-4146-ae8e-b41fb8ff6945"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Delete an invitation to join a shared project.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>invitation_id <em>String</em></td>
<td>Yes</td>
<td>The invitation to be deleted.</td>
</tr>
</tbody></table>

<blockquote>
<p>An example section object</p>
</blockquote>
<pre><code><span class="token punctuation">{</span>
    <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"6Jf8VQXxpwv56VQ7"</span><span class="token punctuation">,</span>
    <span class="token string">"name"</span><span class="token punctuation">:</span> <span class="token string">"Groceries"</span><span class="token punctuation">,</span>
    <span class="token string">"project_id"</span><span class="token punctuation">:</span> <span class="token string">"9Bw8VQXxpwv56ZY2"</span><span class="token punctuation">,</span>
    <span class="token string">"section_order"</span><span class="token punctuation">:</span> <span class="token number">1</span><span class="token punctuation">,</span>
    <span class="token string">"is_collapsed"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"user_id"</span><span class="token punctuation">:</span> <span class="token string">"2671355"</span><span class="token punctuation">,</span>
    <span class="token string">"is_deleted"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"is_archived"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"archived_at"</span><span class="token punctuation">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
    <span class="token string">"added_at"</span><span class="token punctuation">:</span> <span class="token string">"2019-10-07T07:09:27.000000Z"</span><span class="token punctuation">,</span>
    <span class="token string">"updated_at"</span><span class="token punctuation">:</span> <span class="token string">"2019-10-07T07:09:27.000000Z"</span>
<span class="token punctuation">}</span>
</code></pre>
<h4 id="properties">Properties</h4>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>The ID of the section.</td>
</tr>
<tr>
<td>name <em>String</em></td>
<td>The name of the section.</td>
</tr>
<tr>
<td>project_id <em>String</em></td>
<td>Project that the section resides in</td>
</tr>
<tr>
<td>section_order <em>Integer</em></td>
<td>The order of the section. Defines the position of the section among all the sections in the project.</td>
</tr>
<tr>
<td>is_collapsed <em>Boolean</em></td>
<td>Whether the section&#39;s tasks are collapsed (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>sync_id <em>String</em></td>
<td>A special ID for shared sections (a number or <code>null</code> if not set). Used internally and can be ignored.</td>
</tr>
<tr>
<td>is_deleted <em>Boolean</em></td>
<td>Whether the section is marked as deleted (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>is_archived <em>Boolean</em></td>
<td>Whether the section is marked as archived (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>archived_at <em>String</em></td>
<td>The date when the section was archived (or <code>null</code> if not archived).</td>
</tr>
<tr>
<td>added_at <em>String</em></td>
<td>The date when the section was created.</td>
</tr>
<tr>
<td>updated_at <em>String</em></td>
<td>The date when the section was updated.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example add section request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[{
      "type": "section_add",
      "temp_id": "69ca86df-5ffe-4be4-9c3a-ad14fe98a58a",
      "uuid": "97b68ab2-f524-48da-8288-476a42cccf28",
      "args": {"name": "Groceries", "project_id": "9Bw8VQXxpwv56ZY2"}
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"97b68ab2-f524-48da-8288-476a42cccf28"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token string">"temp_id_mapping"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"69ca86df-5ffe-4be4-9c3a-ad14fe98a58a"</span><span class="token builtin class-name">:</span> <span class="token string">"6X7FxXvX84jHphx2"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Add a new section to a project.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>name <em>String</em></td>
<td>Yes</td>
<td>The name of the section.</td>
</tr>
<tr>
<td>project_id <em>String</em></td>
<td>Yes</td>
<td>The ID of the parent project.</td>
</tr>
<tr>
<td>section_order <em>Integer</em></td>
<td>No</td>
<td>The order of the section. Defines the position of the section among all the sections in the project.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example update section request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[{
      "type": "section_update",
      "uuid": "afda2f29-319c-4d09-8162-f2975bed3124",
      "args": {"id": "6X7FxXvX84jHphx2", "name": "Supermarket"}
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"afda2f29-319c-4d09-8162-f2975bed3124"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Updates section attributes.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>The ID of the section.</td>
</tr>
<tr>
<td>name <em>String</em></td>
<td>No</td>
<td>The name of the section.</td>
</tr>
<tr>
<td>is_collapsed <em>Boolean</em></td>
<td>No</td>
<td>Whether the section&#39;s tasks are collapsed (a <code>true</code> or <code>false</code> value).</td>
</tr>
</tbody></table>

<blockquote>
<p>Example move section request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[{
      "type": "section_move",
      "uuid": "a8583f66-5885-4729-9e55-462e72d685ff",
      "args": {"id": "6X7FxXvX84jHphx2", "project_id": "9Bw8VQXxpwv56ZY2"}
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"a8583f66-5885-4729-9e55-462e72d685ff"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Move section to a different location.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>The ID of the section.</td>
</tr>
<tr>
<td>project_id <em>String</em></td>
<td>No</td>
<td>ID of the destination project.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example reorder sections request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[{
      "type": "section_reorder",
      "uuid": "109af205-6ff7-47fa-a5f8-1f13e59744ef",
      "args": {
        "sections": [
          {"id": "6X7FxXvX84jHphx2", "section_order": 1},
          {"id": "6X9FxXvX64jjphx3", "section_order": 2}
        ]
      }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"109af205-6ff7-47fa-a5f8-1f13e59744ef"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>The command updates <code>section_order</code> properties of sections in bulk.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>sections <em>Array of Objects</em></td>
<td>Yes</td>
<td>An array of objects to update. Each object contains two attributes: <code>id</code> of the section to update and <code>section_order</code>, the new order.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example delete section request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[{
      "type": "section_delete",
      "uuid": "cebb5267-3e3c-40da-af44-500649281936",
      "args": {"id": "6X7FxXvX84jHphx2"}
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"cebb5267-3e3c-40da-af44-500649281936"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Delete a section and all its child tasks.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>ID of the section to delete.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example archive section request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[{
      "type": "section_archive",
      "uuid": "2451f267-46ab-4f0e-8db7-82a9cd576f72",
      "args": {"id": "6X7FxXvX84jHphx2"}
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"2451f267-46ab-4f0e-8db7-82a9cd576f72"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Archive a section and all its child tasks.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>Section ID to archive.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example unarchive section request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[{
      "type": "section_unarchive",
      "uuid": "cd3a4b4b-182e-4733-b6b5-20a621ba98b8",
      "args": {"id": "6X7FxXvX84jHphx2"}
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"cd3a4b4b-182e-4733-b6b5-20a621ba98b8"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>This command is used to unarchive a section.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>Section ID to unarchive</td>
</tr>
</tbody></table>

<blockquote>
<p>An example reminder:</p>
</blockquote>
<pre><code><span class="token punctuation">{</span>
  <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"6X7Vfq5rqPMM5j5q"</span><span class="token punctuation">,</span>
  <span class="token string">"notify_uid"</span><span class="token punctuation">:</span> <span class="token string">"2671355"</span><span class="token punctuation">,</span>
  <span class="token string">"item_id"</span><span class="token punctuation">:</span> <span class="token string">"6RP2hmPwM3q4WGfW"</span><span class="token punctuation">,</span>
  <span class="token string">"type"</span><span class="token punctuation">:</span> <span class="token string">"absolute"</span><span class="token punctuation">,</span>
  <span class="token string">"due"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span>
    <span class="token string">"date"</span><span class="token punctuation">:</span> <span class="token string">"2016-08-05T07:00:00.000000Z"</span><span class="token punctuation">,</span>
    <span class="token string">"timezone"</span><span class="token punctuation">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
    <span class="token string">"is_recurring"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"string"</span><span class="token punctuation">:</span> <span class="token string">"tomorrow at 10:00"</span><span class="token punctuation">,</span>
    <span class="token string">"lang"</span><span class="token punctuation">:</span> <span class="token string">"en"</span>
  <span class="token punctuation">}</span><span class="token punctuation">,</span>
  <span class="token string">"minute_offset"</span><span class="token punctuation">:</span> <span class="token number">180</span><span class="token punctuation">,</span>
  <span class="token string">"is_deleted"</span><span class="token punctuation">:</span> <span class="token boolean">false</span>
<span class="token punctuation">}</span>
</code></pre>
<p><em>Availability of reminders functionality and the maximum number of stored reminders are dependent
on the current user plan. These values are indicated by the <code>reminders</code>, <code>max_reminders_time</code> and
<code>max_reminders_location</code> properties of the <a href="#tag/Sync/User/User-plan-limits">user plan limits</a> object.</em></p>
<h4 id="properties">Properties</h4>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>The ID of the reminder.</td>
</tr>
<tr>
<td>notify_uid <em>String</em></td>
<td>The user ID which should be notified of the reminder, typically the current user ID creating the reminder.</td>
</tr>
<tr>
<td>item_id <em>String</em></td>
<td>The item ID for which the reminder is about.</td>
</tr>
<tr>
<td>type <em>String</em></td>
<td>The type of the reminder: <code>relative</code> for a time-based reminder specified in minutes from now, <code>absolute</code> for a time-based reminder with a specific time and date in the future, and <code>location</code> for a location-based reminder.</td>
</tr>
<tr>
<td>due <em>Object</em></td>
<td>The due date of the reminder. See the <a href="#tag/Due-dates">Due dates</a> section for more details. Note that reminders only support due dates with time, since full-day reminders don&#39;t make sense.</td>
</tr>
<tr>
<td>minute_offset <em>Integer</em></td>
<td>The relative time in minutes before the due date of the item, in which the reminder should be triggered. Note that the item should have a due date with time set in order to add a relative reminder.</td>
</tr>
<tr>
<td>name <em>String</em></td>
<td>An alias name for the location.</td>
</tr>
<tr>
<td>loc_lat <em>String</em></td>
<td>The location latitude.</td>
</tr>
<tr>
<td>loc_long <em>String</em></td>
<td>The location longitude.</td>
</tr>
<tr>
<td>loc_trigger <em>String</em></td>
<td>What should trigger the reminder: <code>on_enter</code> for entering the location, or <code>on_leave</code> for leaving the location.</td>
</tr>
<tr>
<td>radius <em>Integer</em></td>
<td>The radius around the location that is still considered as part of the location (in meters).</td>
</tr>
<tr>
<td>is_deleted <em>Boolean</em></td>
<td>Whether the reminder is marked as deleted (a <code>true</code> or <code>false</code> value).</td>
</tr>
</tbody></table>

<blockquote>
<p>Example of adding relative reminder:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "reminder_add",
        "temp_id": "e24ad822-a0df-4b7d-840f-83a5424a484a",
        "uuid": "41e59a76-3430-4e44-92b9-09d114be0d49",
        "args": {
            "item_id": "6RP2hmPwM3q4WGfW",
            "minute_offset": 30,
            "type": "absolute"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"41e59a76-3430-4e44-92b9-09d114be0d49"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token string">"temp_id_mapping"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"e24ad822-a0df-4b7d-840f-83a5424a484a"</span><span class="token builtin class-name">:</span> <span class="token string">"2992683215"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<blockquote>
<p>Example of adding an absolute reminder:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "reminder_add",
        "temp_id": "952a365e-4965-4113-b4f4-80cdfcada172u",
        "uuid": "e7c8be2d-f484-4852-9422-a9984c58b1cd",
        "args": {
            "item_id": "6RP2hmPwM3q4WGfW",
            "due": {
                "date": "2014-10-15T11:00:00.000000Z"
            },
            "type": "absolute"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"e7c8be2d-f484-4852-9422-a9984c58b1cd"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token string">"temp_id_mapping"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"952a365e-4965-4113-b4f4-80cdfcada172"</span><span class="token builtin class-name">:</span> <span class="token string">"2992683215"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<blockquote>
<p>Example of adding a location reminder:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "reminder_add",
        "temp_id": "7ad9609d-579f-4828-95c5-3600acdb2c81",
        "uuid": "830cf409-daba-479c-a624-68eb0c07d01c",
        "args": {
            "item_id": "6RP2hmPwM3q4WGfW",
            "type": "location",
            "name": "Aliados",
            "loc_lat": "41.148581",
            "loc_long":"-8.610945000000015",
            "loc_trigger":"on_enter",
            "radius": 100
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"830cf409-daba-479c-a624-68eb0c07d01c"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token string">"temp_id_mapping"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"7ad9609d-579f-4828-95c5-3600acdb2c81"</span><span class="token builtin class-name">:</span> <span class="token string">"2992683215"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Add a new reminder to the user account related to the API credentials.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>item_id <em>String</em></td>
<td>Yes</td>
<td>The item ID for which the reminder is about.</td>
</tr>
<tr>
<td>type <em>String</em></td>
<td>Yes</td>
<td>The type of the reminder: <code>relative</code> for a time-based reminder specified in minutes from now, <code>absolute</code> for a time-based reminder with a specific time and date in the future, and <code>location</code> for a location-based reminder.</td>
</tr>
<tr>
<td>notify_uid <em>String</em></td>
<td>No</td>
<td>The user ID which should be notified of the reminder, typically the current user ID creating the reminder.</td>
</tr>
<tr>
<td>due <em>Object</em></td>
<td>No</td>
<td>The due date of the reminder. See the <a href="#tag/Due-dates">Due dates</a> section for more details. Note that reminders only support due dates with time, since full-day reminders don&#39;t make sense.</td>
</tr>
<tr>
<td>minute_offset <em>Integer</em></td>
<td>No</td>
<td>The relative time in minutes before the due date of the item, in which the reminder should be triggered. Note, that the item should have a due date with time set in order to add a relative reminder.</td>
</tr>
<tr>
<td>name <em>String</em></td>
<td>No</td>
<td>An alias name for the location.</td>
</tr>
<tr>
<td>loc_lat <em>String</em></td>
<td>No</td>
<td>The location latitude.</td>
</tr>
<tr>
<td>loc_long <em>String</em></td>
<td>No</td>
<td>The location longitude.</td>
</tr>
<tr>
<td>loc_trigger <em>String</em></td>
<td>No</td>
<td>What should trigger the reminder: <code>on_enter</code> for entering the location, or <code>on_leave</code> for leaving the location.</td>
</tr>
<tr>
<td>radius <em>Integer</em></td>
<td>No</td>
<td>The radius around the location that is still considered as part of the location (in meters).</td>
</tr>
</tbody></table>

<blockquote>
<p>Example update reminder request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "reminder_update",
        "uuid": "b0e7562e-ea9f-4c84-87ee-9cbf9c103234",
        "args": {
            "id": "6X7VrXrqjX6642cv",
            "due": {
                "date": "2014-10-10T15:00:00.000000"
            }
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"b0e7562e-ea9f-4c84-87ee-9cbf9c103234"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Update a reminder from the user account related to the API credentials.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>The ID of the reminder.</td>
</tr>
<tr>
<td>notify_uid <em>String</em></td>
<td>No</td>
<td>The user ID which should be notified of the reminder, typically the current user ID creating the reminder.</td>
</tr>
<tr>
<td>type <em>String</em></td>
<td>No</td>
<td>The type of the reminder: <code>relative</code> for a time-based reminder specified in minutes from now, <code>absolute</code> for a time-based reminder with a specific time and date in the future, and <code>location</code> for a location-based reminder.</td>
</tr>
<tr>
<td>due <em>Object</em></td>
<td>No</td>
<td>The due date of the reminder. See the <a href="#tag/Due-dates">Due dates</a> section for more details. Note that reminders only support due dates with time, since full-day reminders don&#39;t make sense.</td>
</tr>
<tr>
<td>minute_offset <em>Integer</em></td>
<td>No</td>
<td>The relative time in minutes before the due date of the item, in which the reminder should be triggered. Note, that the item should have a due date with time set in order to add a relative reminder.</td>
</tr>
<tr>
<td>name <em>String</em></td>
<td>No</td>
<td>An alias name for the location.</td>
</tr>
<tr>
<td>loc_lat <em>String</em></td>
<td>No</td>
<td>The location latitude.</td>
</tr>
<tr>
<td>loc_long <em>String</em></td>
<td>No</td>
<td>The location longitude.</td>
</tr>
<tr>
<td>loc_trigger <em>String</em></td>
<td>No</td>
<td>What should trigger the reminder: <code>on_enter</code> for entering the location, or <code>on_leave</code> for leaving the location.</td>
</tr>
<tr>
<td>radius <em>Integer</em></td>
<td>No</td>
<td>The radius around the location that is still considered as part of the location (in meters).</td>
</tr>
</tbody></table>

<blockquote>
<p>Example delete reminder request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "reminder_delete",
        "uuid": "0896d03b-eb90-49f7-9020-5ed3fd09df2d",
        "args": {"id": "6X7VrXrqjX6642cv"}
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"0896d03b-eb90-49f7-9020-5ed3fd09df2d"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Delete a reminder from the current user account.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>The ID of the filter.</td>
</tr>
</tbody></table>

<p>Locations are a top-level entity in the sync model. They contain a list of all
locations that are used within user&#39;s current location reminders.</p>
<blockquote>
<p>An example location object</p>
</blockquote>
<pre><code><span class="token punctuation">[</span><span class="token string">"Shibuya-ku, Japan"</span><span class="token punctuation">,</span> <span class="token string">"35.6623001098633"</span><span class="token punctuation">,</span> <span class="token string">"139.706527709961"</span><span class="token punctuation">]</span>
</code></pre>
<h4 id="properties">Properties</h4>
<p>The location object is specific, as it&#39;s not an object, but an ordered array.</p>
<table>
<thead>
<tr>
<th>Array index</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>0 <em>String</em></td>
<td>Name of the location.</td>
</tr>
<tr>
<td>1 <em>String</em></td>
<td>Location latitude.</td>
</tr>
<tr>
<td>2 <em>String</em></td>
<td>Location longitude.</td>
</tr>
</tbody></table>
<h3 id="clear-locations">Clear locations</h3>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[{"type": "clear_locations", "uuid": "d285ae02-80c6-477c-bfa9-45272d7bddfb", "args": {}}]'</span>

<span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"d285ae02-80c6-477c-bfa9-45272d7bddfb"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Clears the locations list, which is used for location reminders.</p>

<blockquote>
<p>An example project object:</p>
</blockquote>
<pre><code><span class="token punctuation">{</span>
    <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"6Jf8VQXxpwv56VQ7"</span><span class="token punctuation">,</span>
    <span class="token string">"name"</span><span class="token punctuation">:</span> <span class="token string">"Shopping List"</span><span class="token punctuation">,</span>
    <span class="token string">"description"</span><span class="token punctuation">:</span> <span class="token string">"Stuff to buy"</span><span class="token punctuation">,</span>
    <span class="token string">"workspace_id"</span><span class="token punctuation">:</span> <span class="token number">12345</span><span class="token punctuation">,</span>
    <span class="token string">"is_invite_only"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"status"</span><span class="token punctuation">:</span> <span class="token string">"IN_PROGRESS"</span><span class="token punctuation">,</span>
    <span class="token string">"is_link_sharing_enabled"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"collaborator_role_default"</span><span class="token punctuation">:</span> <span class="token string">"READ_WRITE"</span><span class="token punctuation">,</span>
    <span class="token string">"color"</span><span class="token punctuation">:</span> <span class="token string">"lime_green"</span><span class="token punctuation">,</span>
    <span class="token string">"parent_id"</span><span class="token punctuation">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
    <span class="token string">"child_order"</span><span class="token punctuation">:</span> <span class="token number">1</span><span class="token punctuation">,</span>
    <span class="token string">"is_collapsed"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"shared"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"can_assign_tasks"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"is_deleted"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"is_archived"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"is_favorite"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"is_frozen"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"view_style"</span><span class="token punctuation">:</span> <span class="token string">"list"</span><span class="token punctuation">,</span>
    <span class="token string">"role"</span><span class="token punctuation">:</span> <span class="token string">"READ_WRITE"</span>
    <span class="token string">"inbox_project"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"folder_id"</span><span class="token punctuation">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
    <span class="token string">"created_at"</span><span class="token punctuation">:</span> <span class="token string">"2023-07-13T10:20:59Z"</span><span class="token punctuation">,</span>
    <span class="token string">"updated_at"</span><span class="token punctuation">:</span> <span class="token string">"2024-12-10T13:27:29Z"</span><span class="token punctuation">,</span>
    "is_pending_default_collaborator_invites<span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
<span class="token punctuation">}</span>
</code></pre>
<h4 id="properties">Properties</h4>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>The ID of the project.</td>
</tr>
<tr>
<td>name <em>String</em></td>
<td>The name of the project.</td>
</tr>
<tr>
<td>description <em>String</em></td>
<td>Description for the project. <em>Only used for teams</em></td>
</tr>
<tr>
<td>workspace_id <em>String</em></td>
<td>Real or temp ID of the workspace the project. <em>Only used for teams</em></td>
</tr>
<tr>
<td>is_invite_only <em>Boolean</em></td>
<td>Indicates if the project is invite-only or if it should be visible for everyone in the workspace. If missing or null, the default value from the workspace <code>is_invite_only_default</code> will be used. <em>Only used for teams</em></td>
</tr>
<tr>
<td>status <em>String</em></td>
<td>The status of the project. Possible values: <code>PLANNED</code>, <code>IN_PROGRESS</code>, <code>PAUSED</code>, <code>COMPLETED</code>, <code>CANCELED</code>. <em>Only used for teams</em></td>
</tr>
<tr>
<td>is_link_sharing_enabled <em>Boolean</em></td>
<td>If False, the project is invite-only and people can&#39;t join by link. If true, the project is visible to anyone with a link, and anyone can join it. <em>Only used for teams</em></td>
</tr>
<tr>
<td>collaborator_role_default <em>String</em></td>
<td>The role a user can have. Possible values: <code>CREATOR</code>, <code>ADMIN</code>, <code>READ_WRITE</code>, <code>EDIT_ONLY</code>, <code>COMPLETE_ONLY</code>. (<code>CREATOR</code> is equivalent to admin and is automatically set at project creation; it can&#39;t be set to anyone later). Defaults to <code>READ_WRITE</code>. <em>Only used for teams</em></td>
</tr>
<tr>
<td>color <em>String</em></td>
<td>The color of the project icon. Refer to the <code>name</code> column in the <a href="#tag/Colors">Colors</a> guide for more info.</td>
</tr>
<tr>
<td>parent_id <em>String</em></td>
<td>The ID of the parent project. Set to <code>null</code> for root projects.</td>
</tr>
<tr>
<td>child_order <em>Integer</em></td>
<td>The order of the project. Defines the position of the project among all the projects with the same <code>parent_id</code></td>
</tr>
<tr>
<td>is_collapsed <em>Boolean</em></td>
<td>Whether the project&#39;s sub-projects are collapsed (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>shared <em>Boolean</em></td>
<td>Whether the project is shared (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>can_assign_tasks <em>Boolean</em></td>
<td>Whether tasks in the project can be assigned to users (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>is_deleted <em>Boolean</em></td>
<td>Whether the project is marked as deleted (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>is_archived <em>Boolean</em></td>
<td>Whether the project is marked as archived (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>is_favorite <em>Boolean</em></td>
<td>Whether the project is a favorite (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>is_frozen <em>Boolean</em></td>
<td>Workspace or personal projects from a cancelled subscription (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>view_style <em>Enum</em></td>
<td>The mode in which to render tasks in this project. One of <code>list</code>, <code>board</code>, or <code>calendar</code>.</td>
</tr>
<tr>
<td>role <em>String</em></td>
<td>The role of the requesting user. Possible values: <code>CREATOR</code>, <code>ADMIN</code>, <code>READ_WRITE</code>, <code>EDIT_ONLY</code>, <code>COMPLETE_ONLY</code>. <em>Only used for teams</em></td>
</tr>
<tr>
<td>inbox_project <em>Boolean</em></td>
<td>Whether the project is <code>Inbox</code> (<code>true</code> or otherwise this property is not sent).</td>
</tr>
<tr>
<td>folder_id <em>String</em></td>
<td>The ID of the folder which this project is in.</td>
</tr>
<tr>
<td>created_at <em>String</em></td>
<td>Project creation date in the format YYYY-MM-DDTHH:MM:SSZ date.</td>
</tr>
<tr>
<td>updated_at <em>String</em></td>
<td>Project update date in the format YYYY-MM-DDTHH:MM:SSZ date.</td>
</tr>
<tr>
<td>is_pending_default_collaborator_invites <em>Boolean</em></td>
<td>If true, we are still adding default collaborators to the project in background. <em>Only used for teams</em></td>
</tr>
<tr>
<td>access <em>Object</em></td>
<td>Project access configuration. Contains <code>visibility</code> (<code>&quot;restricted&quot;</code>, <code>&quot;team&quot;</code>, or <code>&quot;public&quot;</code>) and <code>configuration</code> object. For public projects, configuration includes <code>hide_collaborator_details</code> and <code>disable_duplication</code> booleans. <em>Only used for teams</em></td>
</tr>
</tbody></table>
<p><strong>Note:</strong> <code>project.view_style</code> takes precedence over
<a href="#tag/Sync/View-Options"><code>view_options.view_mode</code></a> for projects in Todoist
clients. The former is set per project, while the latter is set per user.</p>

<blockquote>
<p>Example add project request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "project_add",
        "temp_id": "4ff1e388-5ca6-453a-b0e8-662ebf373b6b",
        "uuid": "32774db9-a1da-4550-8d9d-910372124fa4",
        "args": {
            "name": "Shopping List"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"32774db9-a1da-4550-8d9d-910372124fa4"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token string">"temp_id_mapping"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"4ff1e388-5ca6-453a-b0e8-662ebf373b6b"</span><span class="token builtin class-name">:</span> <span class="token string">"6Jf8VQXxpwv56VQ7"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Add a new project.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>name <em>String</em></td>
<td>Yes</td>
<td>The name of the project (a string value).</td>
</tr>
<tr>
<td>color <em>String</em></td>
<td>No</td>
<td>The color of the project icon. Refer to the <code>name</code> column in the <a href="#tag/Colors">Colors</a> guide for more info.</td>
</tr>
<tr>
<td>parent_id <em>String</em></td>
<td>No</td>
<td>The ID of the parent project. Set to <code>null</code> for root projects</td>
</tr>
<tr>
<td>folder_id <em>String</em></td>
<td>No</td>
<td>The ID of the folder, when creating projects in workspaces. Set to <code>null</code> for root projects</td>
</tr>
<tr>
<td>child_order <em>Integer</em></td>
<td>No</td>
<td>The order of the project. Defines the position of the project among all the projects with the same <code>parent_id</code></td>
</tr>
<tr>
<td>is_favorite <em>Boolean</em></td>
<td>No</td>
<td>Whether the project is a favorite (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>view_style <em>String</em></td>
<td>No</td>
<td>A string value (either <code>list</code> or <code>board</code>, default is <code>list</code>). This determines the way the project is displayed within the Todoist clients.</td>
</tr>
<tr>
<td>description <em>String</em></td>
<td>No</td>
<td>Description for the project (up to 1024 characters). <em>Only used for teams</em></td>
</tr>
<tr>
<td>workspace_id <em>String</em></td>
<td>No</td>
<td>Real or temp ID of the workspace the project should belong to</td>
</tr>
<tr>
<td>is_invite_only <em>Boolean</em></td>
<td>No</td>
<td>Indicates if the project is invite-only or if it should be visible for everyone in the workspace. If missing or null, the default value from the workspace <code>is_invite_only_default</code> will be used. <em>Only used for teams</em></td>
</tr>
<tr>
<td>status <em>String</em></td>
<td>No</td>
<td>The status of the project. Possible values: <code>PLANNED</code>, <code>IN_PROGRESS</code>, <code>PAUSED</code>, <code>COMPLETED</code>, <code>CANCELED</code>. <em>Only used for teams</em></td>
</tr>
<tr>
<td>is_link_sharing_enabled <em>Boolean</em></td>
<td>No</td>
<td>If False, the project is invite-only and people can&#39;t join by link. If true, the project is visible to anyone with a link, and anyone can join it. <em>Only used for teams</em></td>
</tr>
<tr>
<td>collaborator_role_default <em>String</em></td>
<td>No</td>
<td>The role a user can have. Possible values: <code>CREATOR</code>, <code>ADMIN</code>, <code>READ_WRITE</code>, <code>EDIT_ONLY</code>, <code>COMPLETE_ONLY</code>. (<code>CREATOR</code> is equivalent to admin and is automatically set at project creation; it can&#39;t be set to anyone later). <em>Only used for teams</em></td>
</tr>
<tr>
<td>access <em>Object</em></td>
<td>No</td>
<td>Project access configuration with <code>visibility</code> (<code>&quot;restricted&quot;</code>, <code>&quot;team&quot;</code>, or <code>&quot;public&quot;</code>) and <code>configuration</code> object. For public projects, configuration includes <code>hide_collaborator_details</code> and <code>disable_duplication</code> booleans. <em>Only used for teams</em></td>
</tr>
</tbody></table>

<blockquote>
<p>Example update project request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token punctuation">[</span>
    <span class="token punctuation">{</span>
        <span class="token string">"type"</span><span class="token builtin class-name">:</span> <span class="token string">"project_update"</span>,
        <span class="token string">"uuid"</span><span class="token builtin class-name">:</span> <span class="token string">"1ca42128-d12f-4a66-8413-4d6ff2838fde"</span>,
        <span class="token string">"args"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
            <span class="token string">"id"</span><span class="token builtin class-name">:</span> <span class="token string">"6Jf8VQXxpwv56VQ7"</span>,
            <span class="token string">"name"</span><span class="token builtin class-name">:</span> <span class="token string">"Shopping"</span>
        <span class="token punctuation">}</span>
    <span class="token punctuation">}</span><span class="token punctuation">]</span>'
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"1ca42128-d12f-4a66-8413-4d6ff2838fde"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Update an existing project.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>The ID of the project (could be temp id).</td>
</tr>
<tr>
<td>name <em>String</em></td>
<td>No</td>
<td>The name of the project (a string value).</td>
</tr>
<tr>
<td>color <em>String</em></td>
<td>No</td>
<td>The color of the project icon. Refer to the <code>name</code> column in the <a href="#tag/Colors">Colors</a> guide for more info.</td>
</tr>
<tr>
<td>is_collapsed <em>Boolean</em></td>
<td>No</td>
<td>Whether the project&#39;s sub-projects are collapsed (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>is_favorite <em>Boolean</em></td>
<td>No</td>
<td>Whether the project is a favorite (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>view_style <em>String</em></td>
<td>No</td>
<td>A string value (either <code>list</code> or <code>board</code>). This determines the way the project is displayed within the Todoist clients.</td>
</tr>
<tr>
<td>description <em>String</em></td>
<td>No</td>
<td>Description for the project (up to 1024 characters). <em>Only used for teams</em></td>
</tr>
<tr>
<td>status <em>String</em></td>
<td>No</td>
<td>The status of the project. Possible values: <code>PLANNED</code>, <code>IN_PROGRESS</code>, <code>PAUSED</code>, <code>COMPLETED</code>, <code>CANCELED</code>. <em>Only used for teams</em></td>
</tr>
<tr>
<td>is_link_sharing_enabled <em>Boolean</em></td>
<td>No</td>
<td>If False, the project is invite-only and people can&#39;t join by link. If true, the project is visible to anyone with a link, and anyone can join it. <em>Only used for teams</em></td>
</tr>
<tr>
<td>collaborator_role_default <em>String</em></td>
<td>No</td>
<td>The role a user can have. Possible values: <code>CREATOR</code>, <code>ADMIN</code>, <code>READ_WRITE</code>, <code>EDIT_ONLY</code>, <code>COMPLETE_ONLY</code>. (<code>CREATOR</code> is equivalent to admin and is automatically set at project creation; it can&#39;t be set to anyone later). Defaults to <code>READ_WRITE</code>. <em>Only used for teams</em></td>
</tr>
<tr>
<td>access <em>Object</em></td>
<td>No</td>
<td>Project access configuration with <code>visibility</code> (<code>&quot;restricted&quot;</code>, <code>&quot;team&quot;</code>, or <code>&quot;public&quot;</code>) and <code>configuration</code> object. For public projects, configuration includes <code>hide_collaborator_details</code> and <code>disable_duplication</code> booleans. <em>Only used for teams</em></td>
</tr>
</tbody></table>

<blockquote>
<p>Example move project request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token punctuation">[</span>
    <span class="token punctuation">{</span>
        <span class="token string">"type"</span><span class="token builtin class-name">:</span> <span class="token string">"project_move"</span>,
        <span class="token string">"uuid"</span><span class="token builtin class-name">:</span> <span class="token string">"1ca42128-d12f-4a66-8413-4d6ff2838fde"</span>,
        <span class="token string">"args"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
            <span class="token string">"id"</span><span class="token builtin class-name">:</span> <span class="token string">"6Jf8VQXxpwv56VQ7"</span>,
            <span class="token string">"parent_id"</span><span class="token builtin class-name">:</span> <span class="token string">"6X7fphhgwcXVGccJ"</span>
        <span class="token punctuation">}</span>
    <span class="token punctuation">}</span><span class="token punctuation">]</span>'
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"1ca42128-d12f-4a66-8413-4d6ff2838fde"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Update parent project relationships of the project.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>The ID of the project (could be a temp id).</td>
</tr>
<tr>
<td>parent_id <em>String</em></td>
<td>No</td>
<td>The ID of the parent project (could be a temp id). If set to null, the project will be moved to the root.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example move project to workspace request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token punctuation">[</span>
    <span class="token punctuation">{</span>
        <span class="token string">"type"</span><span class="token builtin class-name">:</span> <span class="token string">"project_move_to_workspace"</span>,
        <span class="token string">"uuid"</span><span class="token builtin class-name">:</span> <span class="token string">"1ca42128-d12f-4a66-8413-4d6ff2838fde"</span>,
        <span class="token string">"args"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
            <span class="token string">"project_id"</span><span class="token builtin class-name">:</span> <span class="token string">"6Jf8VQXxpwv56VQ7"</span>,
            <span class="token string">"workspace_id"</span><span class="token builtin class-name">:</span> <span class="token string">"220325187"</span>,
            <span class="token string">"is_invite_only"</span><span class="token builtin class-name">:</span> false,
            <span class="token string">"folder_id"</span><span class="token builtin class-name">:</span> null
        <span class="token punctuation">}</span>
    <span class="token punctuation">}</span><span class="token punctuation">]</span>'
</code></pre>
<blockquote>
<p>Example response (with <code>use_lro=true</code>):</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
    <span class="token string">"1ca42128-d12f-4a66-8413-4d6ff2838fde"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
      <span class="token string">"operation"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
        <span class="token string">"id"</span><span class="token builtin class-name">:</span> <span class="token string">"2147483707"</span>,
        <span class="token string">"operation_type"</span><span class="token builtin class-name">:</span> <span class="token string">"MOVE_PROJECT_TO_WORKSPACE"</span>,
        <span class="token string">"status"</span><span class="token builtin class-name">:</span> <span class="token string">"ONGOING"</span>,
        <span class="token string">"args"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
          <span class="token string">"project_id"</span><span class="token builtin class-name">:</span> <span class="token string">"6Jf8VQXxpwv56VQ7"</span>,
          <span class="token string">"workspace_id"</span><span class="token builtin class-name">:</span> <span class="token string">"220325187"</span>
        <span class="token punctuation">}</span>,
        <span class="token string">"error"</span><span class="token builtin class-name">:</span> null,
        <span class="token string">"created_at"</span><span class="token builtin class-name">:</span> <span class="token string">"2026-01-15T12:00:00.000000Z"</span>,
        <span class="token string">"updated_at"</span><span class="token builtin class-name">:</span> <span class="token string">"2026-01-15T12:00:00.000000Z"</span>
      <span class="token punctuation">}</span>
    <span class="token punctuation">}</span>
  <span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Moves a personal project into the target workspace.</p>
<p>When <code>use_lro=true</code> is passed, this command creates a Long Running Operation (LRO) and
processes the move in the background. The <code>sync_status</code> will contain an <code>operation</code>
object instead of <code>&quot;ok&quot;</code>.</p>
<p><strong>Deprecation notice:</strong> The synchronous behavior (without <code>use_lro</code>) is deprecated and
will be removed in a future version. In that version, this command will always use LRO.
Clients should migrate to using <code>use_lro=true</code>.</p>
<p>A few notes about moving projects to a workspace:</p>
<ul>
<li>Moving a parent project to a workspace will also move all its child projects to that workspace.</li>
<li>If no folder_id is supplied, child projects will be moved into a folder with the same name as the parent project being moved</li>
<li>If a folder_id is supplied, the parent and child projects will be moved into that folder.</li>
<li>At the moment, it is not possible to move a project to another workspace (changing its <code>workspace_id</code>), or to the user&#39;s personal workspace.</li>
<li>Moving a project to a workspace affects all its collaborators. Collaborators who are not members of the target workspace will be added as guests, if guest members are allowed in the target workspace</li>
</ul>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>project_id <em>String</em></td>
<td>Yes</td>
<td>The ID of the project (can be a temp id).</td>
</tr>
<tr>
<td>workspace_id <em>String</em></td>
<td>Yes</td>
<td>The ID of the workspace the project will be moved into</td>
</tr>
<tr>
<td>is_invite_only <em>Boolean</em></td>
<td>No</td>
<td>If true the project will be restricted access, otherwise any workspace member could join it</td>
</tr>
<tr>
<td>folder_id <em>String</em></td>
<td>No</td>
<td>If supplied, the project and any child projects will be moved into this workspace folder</td>
</tr>
<tr>
<td>use_lro <em>Boolean</em></td>
<td>No</td>
<td>If true, process as a Long Running Operation. Recommended; will become the default behavior in a future version.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example move project out of a workspace request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token punctuation">[</span>
    <span class="token punctuation">{</span>
        <span class="token string">"type"</span><span class="token builtin class-name">:</span> <span class="token string">"project_move_to_personal"</span>,
        <span class="token string">"uuid"</span><span class="token builtin class-name">:</span> <span class="token string">"1ca42128-d12f-4a66-8413-4d6ff2838fde"</span>,
        <span class="token string">"args"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
            <span class="token string">"project_id"</span><span class="token builtin class-name">:</span> <span class="token string">"6Jf8VQXxpwv56VQ7"</span>
        <span class="token punctuation">}</span>
    <span class="token punctuation">}</span><span class="token punctuation">]</span>'
</code></pre>
<blockquote>
<p>Example response (with <code>use_lro=true</code>):</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
    <span class="token string">"1ca42128-d12f-4a66-8413-4d6ff2838fde"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
      <span class="token string">"operation"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
        <span class="token string">"id"</span><span class="token builtin class-name">:</span> <span class="token string">"2147483708"</span>,
        <span class="token string">"operation_type"</span><span class="token builtin class-name">:</span> <span class="token string">"MOVE_PROJECT_TO_PERSONAL"</span>,
        <span class="token string">"status"</span><span class="token builtin class-name">:</span> <span class="token string">"ONGOING"</span>,
        <span class="token string">"args"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
          <span class="token string">"project_id"</span><span class="token builtin class-name">:</span> <span class="token string">"6Jf8VQXxpwv56VQ7"</span>
        <span class="token punctuation">}</span>,
        <span class="token string">"error"</span><span class="token builtin class-name">:</span> null,
        <span class="token string">"created_at"</span><span class="token builtin class-name">:</span> <span class="token string">"2026-01-15T12:00:00.000000Z"</span>,
        <span class="token string">"updated_at"</span><span class="token builtin class-name">:</span> <span class="token string">"2026-01-15T12:00:00.000000Z"</span>
      <span class="token punctuation">}</span>
    <span class="token punctuation">}</span>
  <span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Moves a project inside a workspace out back into a user&#39;s personal space.</p>
<p>When <code>use_lro=true</code> is passed, this command creates a Long Running Operation (LRO) and
processes the move in the background. The <code>sync_status</code> will contain an <code>operation</code>
object instead of <code>&quot;ok&quot;</code>.</p>
<p><strong>Deprecation notice:</strong> The synchronous behavior (without <code>use_lro</code>) is deprecated and
will be removed in a future version. In that version, this command will always use LRO.
We recomment migrating to only rely on <code>use_lro=true</code>.</p>
<p>Only the original creator of the project has permissions to do this, and only if they
are still currently an admin of said workspace.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>project_id <em>String</em></td>
<td>Yes</td>
<td>The ID of the project being moved back (can be a temp id).</td>
</tr>
<tr>
<td>use_lro <em>Boolean</em></td>
<td>No</td>
<td>If true, process as a Long Running Operation. <strong>Recommended</strong>, as it will become the default behavior in a future version.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example leave project request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token punctuation">[</span>
    <span class="token punctuation">{</span>
        <span class="token string">"type"</span><span class="token builtin class-name">:</span> <span class="token string">"project_leave"</span>,
        <span class="token string">"uuid"</span><span class="token builtin class-name">:</span> <span class="token string">"1ca42128-d12f-4a66-8413-4d6ff2838fde"</span>,
        <span class="token string">"args"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
            <span class="token string">"project_id"</span><span class="token builtin class-name">:</span> <span class="token string">"6Jf8VQXxpwv56VQ7"</span>,
        <span class="token punctuation">}</span>
    <span class="token punctuation">}</span><span class="token punctuation">]</span>'
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"1ca42128-d12f-4a66-8413-4d6ff2838fde"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p><em>Only used for teams</em></p>
<p>This command is used to remove self from a workspace project. As this is a
v2-only command, it will only accept v2 project id.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>project_id <em>String</em></td>
<td>Yes</td>
<td>The ID (<code>v2_id</code>) of the project to leave. It only accepts <code>v2_id</code> as the id.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example delete project request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token punctuation">[</span>
    <span class="token punctuation">{</span>
        <span class="token string">"type"</span><span class="token builtin class-name">:</span> <span class="token string">"project_delete"</span>,
        <span class="token string">"uuid"</span><span class="token builtin class-name">:</span> <span class="token string">"367182ba-125f-4dbb-bff6-c1343fd751e4"</span>,
        <span class="token string">"args"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
            <span class="token string">"id"</span><span class="token builtin class-name">:</span> <span class="token string">"6X7fphhgwcXVGccJ"</span>
        <span class="token punctuation">}</span>
    <span class="token punctuation">}</span><span class="token punctuation">]</span>'
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"367182ba-125f-4dbb-bff6-c1343fd751e4"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Delete an existing project and all its descendants.
Workspace projects can only be deleted by <code>ADMIN</code>s and it must be archived first.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>ID of the project to delete (could be a temp id).</td>
</tr>
</tbody></table>

<blockquote>
<p>Example archive project request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token punctuation">[</span>
    <span class="token punctuation">{</span>
        <span class="token string">"type"</span><span class="token builtin class-name">:</span> <span class="token string">"project_archive"</span>,
        <span class="token string">"uuid"</span><span class="token builtin class-name">:</span> <span class="token string">"bbec1a60-2bdd-48ac-a623-c8eb968e1697"</span>,
        <span class="token string">"args"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
            <span class="token string">"id"</span><span class="token builtin class-name">:</span> <span class="token string">"6X7fphhgwcXVGccJ"</span>
        <span class="token punctuation">}</span>
    <span class="token punctuation">}</span><span class="token punctuation">]</span>'
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"bbec1a60-2bdd-48ac-a623-c8eb968e1697"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Archive a project and its descendants.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>ID of the project to archive (could be a temp id).</td>
</tr>
</tbody></table>

<blockquote>
<p>Example unarchive project request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token punctuation">[</span>
    <span class="token punctuation">{</span>
        <span class="token string">"type"</span><span class="token builtin class-name">:</span> <span class="token string">"project_unarchive"</span>,
        <span class="token string">"uuid"</span><span class="token builtin class-name">:</span> <span class="token string">"7d86f042-e098-4fa6-9c1f-a61fe8c39d74"</span>,
        <span class="token string">"args"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
            <span class="token string">"id"</span><span class="token builtin class-name">:</span> <span class="token string">"6X7fphhgwcXVGccJ"</span>
        <span class="token punctuation">}</span>
    <span class="token punctuation">}</span><span class="token punctuation">]</span>'
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"7d86f042-e098-4fa6-9c1f-a61fe8c39d74"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Unarchive a project. No ancestors will be unarchived along with the unarchived
project. Instead, the project is unarchived alone, loses any parent relationship
(becomes a root project), and is placed at the end of the list of other root
projects.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>ID of the project to unarchive (could be a temp id).</td>
</tr>
</tbody></table>

<blockquote>
<p>Example reorder projects request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token punctuation">[</span>
    <span class="token punctuation">{</span>
        <span class="token string">"type"</span><span class="token builtin class-name">:</span> <span class="token string">"project_reorder"</span>,
        <span class="token string">"uuid"</span><span class="token builtin class-name">:</span> <span class="token string">"bf0855a3-0138-4b76-b895-88cad8db9edc"</span>,
        <span class="token string">"args"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
            <span class="token string">"projects"</span><span class="token builtin class-name">:</span> <span class="token punctuation">[</span>
                <span class="token punctuation">{</span>
                    <span class="token string">"id"</span><span class="token builtin class-name">:</span> <span class="token string">"6Jf8VQXxpwv56VQ7"</span>,
                    <span class="token string">"child_order"</span><span class="token builtin class-name">:</span> <span class="token number">1</span>
                <span class="token punctuation">}</span>,
                <span class="token punctuation">{</span>
                    <span class="token string">"id"</span><span class="token builtin class-name">:</span> <span class="token string">"6X7fphhgwcXVGccJ"</span>,
                    <span class="token string">"child_order"</span><span class="token builtin class-name">:</span> <span class="token number">2</span>
                <span class="token punctuation">}</span>
            <span class="token punctuation">]</span>
        <span class="token punctuation">}</span>
    <span class="token punctuation">}</span><span class="token punctuation">]</span>'
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"bf0855a3-0138-4b76-b895-88cad8db9edc"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>The command updates <code>child_order</code> properties of projects in bulk.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>projects <em>Array of Objects</em></td>
<td>Yes</td>
<td>An array of objects to update. Each object contains two attributes: <code>id</code> of the project to update and <code>child_order</code>, the new order.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example change project role request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token punctuation">[</span>
    <span class="token punctuation">{</span>
        <span class="token string">"type"</span><span class="token builtin class-name">:</span> <span class="token string">"project_change_role"</span>,
        <span class="token string">"uuid"</span><span class="token builtin class-name">:</span> <span class="token string">"bbec1a60-2bdd-48ac-a623-c8eb968e1697"</span>,
        <span class="token string">"args"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
            <span class="token string">"id"</span><span class="token builtin class-name">:</span> <span class="token string">"6X7fphhgwcXVGccJ"</span>,
            <span class="token string">"user_id"</span><span class="token builtin class-name">:</span> <span class="token number">12345678</span>,
            <span class="token string">"role"</span><span class="token builtin class-name">:</span> <span class="token string">"EDIT_ONLY"</span>
        <span class="token punctuation">}</span>
    <span class="token punctuation">}</span><span class="token punctuation">]</span>'
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"bbec1a60-2bdd-48ac-a623-c8eb968e1697"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Change the role a project member has within the project.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>ID of the project to change the role for (could be a temp id).</td>
</tr>
<tr>
<td>user_id <em>Int</em></td>
<td>Yes</td>
<td>ID of the user whose role to change.</td>
</tr>
<tr>
<td>role <em>String</em></td>
<td>Yes</td>
<td>New role for the user. Valid values: <code>CREATOR</code>, <code>ADMIN</code>, <code>READ_WRITE</code>, <code>EDIT_ONLY</code>, <code>COMPLETE_ONLY</code>. Note: Only the project creator can be assigned the <code>CREATOR</code> role.</td>
</tr>
</tbody></table>

<blockquote>
<p>An example task comment:</p>
</blockquote>
<pre><code><span class="token punctuation">{</span>
    <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"6X7gfQHG59V8CJJV"</span><span class="token punctuation">,</span>
    <span class="token string">"posted_uid"</span><span class="token punctuation">:</span> <span class="token string">"2671355"</span><span class="token punctuation">,</span>
    <span class="token string">"item_id"</span><span class="token punctuation">:</span> <span class="token string">"6X7gfV9G7rWm5hW8"</span><span class="token punctuation">,</span>
    <span class="token string">"content"</span><span class="token punctuation">:</span> <span class="token string">"Note"</span><span class="token punctuation">,</span>
    <span class="token string">"file_attachment"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span>
        <span class="token string">"file_type"</span><span class="token punctuation">:</span> <span class="token string">"text/plain"</span><span class="token punctuation">,</span>
        <span class="token string">"file_name"</span><span class="token punctuation">:</span> <span class="token string">"File1.txt"</span><span class="token punctuation">,</span>
        <span class="token string">"file_size"</span><span class="token punctuation">:</span> <span class="token number">1234</span><span class="token punctuation">,</span>
        <span class="token string">"file_url"</span><span class="token punctuation">:</span> <span class="token string">"https://example.com/File1.txt"</span><span class="token punctuation">,</span>
        <span class="token string">"upload_state"</span><span class="token punctuation">:</span> <span class="token string">"completed"</span>
    <span class="token punctuation">}</span><span class="token punctuation">,</span>
    <span class="token string">"uids_to_notify"</span><span class="token punctuation">:</span> <span class="token punctuation">[</span>
      <span class="token string">"84129"</span>
    <span class="token punctuation">]</span>
    <span class="token string">"is_deleted"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"posted_at"</span><span class="token punctuation">:</span> <span class="token string">"2014-10-01T14:54:55.000000Z"</span><span class="token punctuation">,</span>
    <span class="token string">"reactions"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span> <span class="token string">"❤️"</span><span class="token punctuation">:</span> <span class="token punctuation">[</span><span class="token string">"2671362"</span><span class="token punctuation">]</span><span class="token punctuation">,</span> <span class="token string">"👍"</span><span class="token punctuation">:</span> <span class="token punctuation">[</span><span class="token string">"2671362"</span><span class="token punctuation">,</span> <span class="token string">"2671366"</span><span class="token punctuation">]</span> <span class="token punctuation">}</span>
<span class="token punctuation">}</span>
</code></pre>
<p><em>Availability of comments functionality is dependent on the current user plan.
This value is indicated by the <code>comments</code> property of the
<a href="#tag/Sync/User/User-plan-limits">user plan limits</a> object.</em></p>
<h4 id="properties">Properties</h4>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>The ID of the note.</td>
</tr>
<tr>
<td>posted_uid <em>String</em></td>
<td>The ID of the user that posted the note.</td>
</tr>
<tr>
<td>item_id <em>String</em></td>
<td>The item which the note is part of.</td>
</tr>
<tr>
<td>content <em>String</em></td>
<td>The content of the note. This value may contain markdown-formatted text and hyperlinks. Details on markdown support can be found in the <a href="https://www.todoist.com/help/articles/format-text-in-a-todoist-task-e5dHw9">Text Formatting article</a> in the Help Center.</td>
</tr>
<tr>
<td>file_attachment <em>Object</em></td>
<td>A file attached to the note (see the <a href="#tag/Sync/Comments/File-Attachments">File Attachments</a> section for details).</td>
</tr>
<tr>
<td>uids_to_notify <em>Array of String</em></td>
<td>A list of user IDs to notify.</td>
</tr>
<tr>
<td>is_deleted <em>Boolean</em></td>
<td>Whether the note is marked as deleted (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>posted_at <em>String</em></td>
<td>The date when the note was posted.</td>
</tr>
<tr>
<td>reactions <em>Object</em></td>
<td>List of emoji reactions and corresponding user IDs.</td>
</tr>
</tbody></table>
<h3 id="add-a-task-comment">Add a task comment</h3>
<blockquote>
<p>Example add comment request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "note_add",
        "temp_id": "59fe4461-287b-4b00-bacc-ee771137a732",
        "uuid": "e1005f08-acd6-4172-bab1-4338f8616e49",
        "args": {
            "item_id": "6X7gfV9G7rWm5hW8",
            "content": "Note1"
        }
    }]'</span>

<span class="token comment"># or adding a comment with a file attached:</span>

$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "note_add",
        "temp_id": "6149e689-1a54-48d6-a8c2-0ee5425554a9",
        "uuid": "554a65e9-56d9-478e-b35b-520c419e37d9",
        "args": {
            "item_id": "6X7gfV9G7rWm5hW8",
            "content": "Note1",
            "file_attachment": {
                "file_type": "image\/gif",
                "file_name": "image.gif",
                "image": "https:\/\/domain\/image.gif",
                "file_url": "https:\/\/domain\/image.gif",
                "image_width":90,
                "image_height":76,
                "file_size":7962
            }
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"e1005f08-acd6-4172-bab1-4338f8616e49"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token string">"temp_id_mapping"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"59fe4461-287b-4b00-bacc-ee771137a732"</span><span class="token builtin class-name">:</span> <span class="token string">"6X7gfQHG59V8CJJV"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>item_id <em>String</em></td>
<td>Yes</td>
<td>The item which the note is part of (a unique number or temp id).</td>
</tr>
<tr>
<td>content <em>String</em></td>
<td>Yes</td>
<td>The content of the note. This value may contain markdown-formatted text and hyperlinks. Details on markdown support can be found in the <a href="https://www.todoist.com/help/articles/format-text-in-a-todoist-task-e5dHw9">Text Formatting article</a> in the Help Center.</td>
</tr>
<tr>
<td>file_attachment <em>Object</em></td>
<td>No</td>
<td>A file attached to the note (see the <a href="#tag/Sync/Comments/File-Attachments">File Attachments</a> section for details, and learn how to upload a file in the <a href="#tag/Uploads">Uploads section</a>).</td>
</tr>
<tr>
<td>uids_to_notify <em>Array of String</em></td>
<td>No</td>
<td>A list of user IDs to notify.</td>
</tr>
</tbody></table>
<h3 id="update-a-task-comment">Update a task comment</h3>
<blockquote>
<p>Example update comment request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "note_update",
        "uuid": "8a38f9c5-2cd0-4da5-87c1-26d617b354e0",
        "args": {
            "id": "6X7gfQHG59V8CJJV",
            "content": "Updated Note1"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"8a38f9c5-2cd0-4da5-87c1-26d617b354e0"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<h4 id="command-arguments-1">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>The ID of the note.</td>
</tr>
<tr>
<td>content <em>String</em></td>
<td>Yes</td>
<td>The content of the note. This value may contain markdown-formatted text and hyperlinks. Details on markdown support can be found in the <a href="https://www.todoist.com/help/articles/format-text-in-a-todoist-task-e5dHw9">Text Formatting article</a> in the Help Center.</td>
</tr>
<tr>
<td>file_attachment <em>Object</em></td>
<td>No</td>
<td>A file attached to the note (see the <a href="#tag/Sync/Comments/File-Attachments">File Attachments</a> section for details, and learn how to upload a file in the <a href="#tag/Uploads">Uploads section</a>).</td>
</tr>
</tbody></table>
<h3 id="delete-a-task-comment">Delete a task comment</h3>
<blockquote>
<p>Example delete comment request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "note_delete",
        "uuid": "8d666fda-73c3-4677-8b04-5d223632c24f",
        "args": {"id": "6X7hH7Gpwr3w7jX9"}
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span> <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"8d666fda-73c3-4677-8b04-5d223632c24f"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<h4 id="command-arguments-2">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>The ID of the note.</td>
</tr>
</tbody></table>

<blockquote>
<p>An example project comment:</p>
</blockquote>
<pre><code><span class="token punctuation">{</span>
    <span class="token string">"content"</span><span class="token punctuation">:</span> <span class="token string">"Hello 2"</span><span class="token punctuation">,</span>
    <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"6X7hH9GWrqWhF69Q"</span><span class="token punctuation">,</span>
    <span class="token string">"is_deleted"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"posted_at"</span><span class="token punctuation">:</span> <span class="token string">"2018-08-14T10:56:50.000000Z"</span><span class="token punctuation">,</span>
    <span class="token string">"posted_uid"</span><span class="token punctuation">:</span> <span class="token string">"2671355"</span><span class="token punctuation">,</span>
    <span class="token string">"project_id"</span><span class="token punctuation">:</span> <span class="token string">"6Jf8VQXxpwv56VQ7"</span><span class="token punctuation">,</span>
    <span class="token string">"reactions"</span><span class="token punctuation">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
    <span class="token string">"uids_to_notify"</span><span class="token punctuation">:</span> <span class="token punctuation">[</span><span class="token string">"2671362"</span><span class="token punctuation">]</span><span class="token punctuation">,</span>
    <span class="token string">"reactions"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span> <span class="token string">"❤️"</span><span class="token punctuation">:</span> <span class="token punctuation">[</span><span class="token string">"2671362"</span><span class="token punctuation">]</span><span class="token punctuation">,</span> <span class="token string">"👍"</span><span class="token punctuation">:</span> <span class="token punctuation">[</span><span class="token string">"2671362"</span><span class="token punctuation">,</span> <span class="token string">"2671366"</span><span class="token punctuation">]</span> <span class="token punctuation">}</span><span class="token punctuation">,</span>
    <span class="token string">"file_attachment"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span>
        <span class="token string">"file_type"</span><span class="token punctuation">:</span> <span class="token string">"text/plain"</span><span class="token punctuation">,</span>
        <span class="token string">"file_name"</span><span class="token punctuation">:</span> <span class="token string">"File1.txt"</span><span class="token punctuation">,</span>
        <span class="token string">"file_size"</span><span class="token punctuation">:</span> <span class="token number">1234</span><span class="token punctuation">,</span>
        <span class="token string">"file_url"</span><span class="token punctuation">:</span> <span class="token string">"https://example.com/File1.txt"</span><span class="token punctuation">,</span>
        <span class="token string">"upload_state"</span><span class="token punctuation">:</span> <span class="token string">"completed"</span>
    <span class="token punctuation">}</span>
<span class="token punctuation">}</span>
</code></pre>
<p><em>Availability of comments functionality is dependent on the current user plan.
This value is indicated by the <code>comments</code> property of the
<a href="#tag/Sync/User/User-plan-limits">user plan limits</a> object.</em></p>
<h4 id="properties">Properties</h4>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>The ID of the note.</td>
</tr>
<tr>
<td>posted_uid <em>Integer</em></td>
<td>The ID of the user that posted the note.</td>
</tr>
<tr>
<td>project_id <em>String</em></td>
<td>The project which the note is part of.</td>
</tr>
<tr>
<td>content <em>String</em></td>
<td>The content of the note. This value may contain markdown-formatted text and hyperlinks. Details on markdown support can be found in the <a href="https://www.todoist.com/help/articles/format-text-in-a-todoist-task-e5dHw9">Text Formatting article</a> in the Help Center.</td>
</tr>
<tr>
<td>file_attachment <em>Object</em></td>
<td>A file attached to the note (see the <a href="#tag/Sync/Comments/File-Attachments">File Attachments</a> section for details).</td>
</tr>
<tr>
<td>uids_to_notify <em>Array of String</em></td>
<td>A list of user IDs to notify.</td>
</tr>
<tr>
<td>is_deleted <em>Boolean</em></td>
<td>Whether the note is marked as deleted (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>posted_at <em>String</em></td>
<td>The date when the note was posted.</td>
</tr>
<tr>
<td>reactions <em>Object</em></td>
<td>List of emoji reactions and corresponding user IDs.</td>
</tr>
</tbody></table>
<h3 id="add-a-project-comment">Add a project comment</h3>
<blockquote>
<p>Example add comment request:</p>
</blockquote>
<pre><code class="language-shell"><span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "note_add",
        "temp_id": "59fe4461-287b-4b00-bacc-ee771137a732",
        "uuid": "e1005f08-acd6-4172-bab1-4338f8616e49",
        "args": {
            "project_id": "6Jf8VQXxpwv56VQ7",
            "content": "Note1"
        }
    }]'</span>

<span class="token comment"># or adding a note with a file attached:</span>

$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "note_add",
        "temp_id": "6149e689-1a54-48d6-a8c2-0ee5425554a9",
        "uuid": "554a65e9-56d9-478e-b35b-520c419e37d9",
        "args": {
            "project_id": "6Jf8VQXxpwv56VQ7",
            "content": "Note1",
            "file_attachment": {
                "file_type": "image\/gif",
                "file_name": "image.gif",
                "image": "https:\/\/domain\/image.gif",
                "file_url": "https:\/\/domain\/image.gif",
                "image_width":90,
                "image_height":76,
                "file_size":7962
            }
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"e1005f08-acd6-4172-bab1-4338f8616e49"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token string">"temp_id_mapping"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"59fe4461-287b-4b00-bacc-ee771137a732"</span><span class="token builtin class-name">:</span> <span class="token string">"6X7hH9GWrqWhF69Q"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>project_id <em>String</em></td>
<td>Yes</td>
<td>The project which the note is part of.</td>
</tr>
<tr>
<td>content <em>String</em></td>
<td>Yes</td>
<td>The content of the note. This value may contain markdown-formatted text and hyperlinks. Details on markdown support can be found in the <a href="https://www.todoist.com/help/articles/format-text-in-a-todoist-task-e5dHw9">Text Formatting article</a> in the Help Center.</td>
</tr>
<tr>
<td>file_attachment <em>Object</em></td>
<td>No</td>
<td>A file attached to the note (see the <a href="#tag/Sync/Comments/File-Attachments">File Attachments</a> section for details, and learn how to upload a file in the <a href="#tag/Uploads">Uploads section</a>).</td>
</tr>
</tbody></table>
<h3 id="update-a-project-comment">Update a project comment</h3>
<blockquote>
<p>Example update comment request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "note_update",
        "uuid": "8a38f9c5-2cd0-4da5-87c1-26d617b354e0",
        "args": {"id": "6X7hH9GWrqWhF69Q", "content": "Updated Note 1"}
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"8a38f9c5-2cd0-4da5-87c1-26d617b354e0"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>The ID of the note.</td>
</tr>
<tr>
<td>content <em>String</em></td>
<td>Yes</td>
<td>The content of the note. This value may contain markdown-formatted text and hyperlinks. Details on markdown support can be found in the <a href="https://www.todoist.com/help/articles/format-text-in-a-todoist-task-e5dHw9">Text Formatting article</a> in the Help Center.</td>
</tr>
<tr>
<td>file_attachment <em>Object</em></td>
<td>No</td>
<td>A file attached to the note (see the <a href="#tag/Sync/Comments/File-Attachments">File Attachments</a> section for details, and learn how to upload a file in the <a href="#tag/Uploads">Uploads section</a>).</td>
</tr>
</tbody></table>
<h3 id="delete-a-project-comment">Delete a project comment</h3>
<blockquote>
<p>Example delete comment request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "note_delete",
        "uuid": "8a38f9c5-2cd0-4da5-87c1-26d617b354e0",
        "args": {"id": "6X7hH9GWrqWhF69Q"}
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"8d666fda-73c3-4677-8b04-5d223632c24f"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<h4 id="command-arguments-1">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>The ID of the note.</td>
</tr>
</tbody></table>

<p>A file attachment is represented as a JSON object. The file attachment may point
to a document previously uploaded by the <code>uploads/add</code> API call, or by any
external resource.</p>
<h4 id="base-file-properties">Base file properties</h4>
<table>
<thead>
<tr>
<th>Attribute</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>file_name <em>String</em></td>
<td>The name of the file.</td>
</tr>
<tr>
<td>file_size <em>Integer</em></td>
<td>The size of the file in bytes.</td>
</tr>
<tr>
<td>file_type <em>String</em></td>
<td>MIME type (for example <code>text/plain</code> or <code>image/png</code>). The <code>file_type</code> is important for Todoist to render the proper preview for the given attachment.</td>
</tr>
<tr>
<td>file_url <em>String</em></td>
<td>The URL where the file is located. Note that we don&#39;t cache the remote content on our servers and stream or expose files directly from third party resources. In particular this means that you should avoid providing links to non-encrypted (plain HTTP) resources, as exposing this files in Todoist may issue a browser warning.</td>
</tr>
<tr>
<td>upload_state <em>String</em></td>
<td>Upload completion state (for example <code>pending</code> or <code>completed</code>).</td>
</tr>
</tbody></table>
<h4 id="image-file-properties">Image file properties</h4>
<p>If you upload an image, you may provide thumbnail paths to ensure Todoist
handles them appropriately. Valid thumbnail information is a JSON array with
URL, width in pixels, height in pixels. Ex.:
[&quot;<a href="https://example.com/img.jpg%22,400,300%5D">https://example.com/img.jpg&quot;,400,300]</a>. &quot;Canonical&quot; thumbnails (ones we create
by <code>uploads/add</code> API call) have the following sizes: <code>96x96</code>, <code>288x288</code>,
<code>528x528</code>.</p>
<table>
<thead>
<tr>
<th>Attribute</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>tn_l <em>List</em></td>
<td>Large thumbnail (a list that contains the URL, the width and the height of the thumbnail).</td>
</tr>
<tr>
<td>tn_m <em>List</em></td>
<td>Medium thumbnail (a list that contains the URL, the width and the height of the thumbnail).</td>
</tr>
<tr>
<td>tn_s <em>List</em></td>
<td>Small thumbnail (a list that contains the URL, the width and the height of the thumbnail).</td>
</tr>
</tbody></table>
<h4 id="audio-file-properties">Audio file properties</h4>
<p>If you upload an audio file, you may provide an extra attribute <code>file_duration</code>
(duration of the audio file in seconds, which takes an integer value). In the
web interface the file is rendered with a <code>&lt;audio&gt;</code> tag, so you should make sure
it&#39;s supported in current web browsers. See
<a href="https://developer.mozilla.org/en-US/docs/Web/Media/Formats">supported media formats</a> for
the reference.</p>

<blockquote>
<p>Examples of live notifications:</p>
</blockquote>
<pre><code><span class="token punctuation">{</span>
    <span class="token string">"created_at"</span><span class="token punctuation">:</span> <span class="token string">"2021-05-10T09:59:36.000000Z"</span><span class="token punctuation">,</span>
    <span class="token string">"is_unread"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"from_uid"</span><span class="token punctuation">:</span> <span class="token string">"2671362"</span><span class="token punctuation">,</span>
    <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"1"</span><span class="token punctuation">,</span>
    <span class="token string">"invitation_id"</span><span class="token punctuation">:</span> <span class="token string">"456"</span><span class="token punctuation">,</span>
    <span class="token string">"invitation_secret"</span><span class="token punctuation">:</span> <span class="token string">"abcdefghijklmno"</span><span class="token punctuation">,</span>
    <span class="token string">"notification_key"</span><span class="token punctuation">:</span> <span class="token string">"notification_123"</span><span class="token punctuation">,</span>
    <span class="token string">"notification_type"</span><span class="token punctuation">:</span> <span class="token string">"share_invitation_sent"</span><span class="token punctuation">,</span>
    <span class="token string">"seq_no"</span><span class="token punctuation">:</span> <span class="token number">12345567890</span><span class="token punctuation">,</span>
    <span class="token string">"state"</span><span class="token punctuation">:</span> <span class="token string">"accepted"</span>
<span class="token punctuation">}</span>

<span class="token punctuation">{</span>
    <span class="token string">"created_at"</span><span class="token punctuation">:</span> <span class="token string">"2021-05-10T09:59:36.000000Z"</span><span class="token punctuation">,</span>
    <span class="token string">"is_unread"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"from_uid"</span><span class="token punctuation">:</span> <span class="token string">"2671362"</span><span class="token punctuation">,</span>
    <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"2"</span><span class="token punctuation">,</span>
    <span class="token string">"invitation_id"</span><span class="token punctuation">:</span> <span class="token string">"456"</span><span class="token punctuation">,</span>
    <span class="token string">"notification_key"</span><span class="token punctuation">:</span> <span class="token string">"notification_123"</span><span class="token punctuation">,</span>
    <span class="token string">"notification_type"</span><span class="token punctuation">:</span> <span class="token string">"share_invitation_accepted"</span><span class="token punctuation">,</span>
    <span class="token string">"project_id"</span><span class="token punctuation">:</span> <span class="token string">"6Jf8VQXxpwv56VQ7"</span><span class="token punctuation">,</span>
    <span class="token string">"seq_no"</span><span class="token punctuation">:</span> <span class="token number">1234567890</span>
<span class="token punctuation">}</span>

<span class="token punctuation">{</span>
    <span class="token string">"created_at"</span><span class="token punctuation">:</span> <span class="token string">"2021-05-10T09:59:36.000000Z"</span><span class="token punctuation">,</span>
    <span class="token string">"is_unread"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"from_uid"</span><span class="token punctuation">:</span> <span class="token string">"2671362"</span><span class="token punctuation">,</span>
    <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"3"</span><span class="token punctuation">,</span>
    <span class="token string">"invitation_id"</span><span class="token punctuation">:</span> <span class="token string">"456"</span><span class="token punctuation">,</span>
    <span class="token string">"notification_key"</span><span class="token punctuation">:</span> <span class="token string">"notification_123"</span><span class="token punctuation">,</span>
    <span class="token string">"notification_type"</span><span class="token punctuation">:</span> <span class="token string">"share_invitation_rejected"</span><span class="token punctuation">,</span>
    <span class="token string">"project_id"</span><span class="token punctuation">:</span> <span class="token string">"6Jf8VQXxpwv56VQ7"</span><span class="token punctuation">,</span>
    <span class="token string">"reject_email"</span><span class="token punctuation">:</span> <span class="token string">"me@example.com"</span><span class="token punctuation">,</span>
    <span class="token string">"seq_no"</span><span class="token punctuation">:</span> <span class="token number">1234567890</span>
<span class="token punctuation">}</span>

<span class="token punctuation">{</span>
    <span class="token string">"created_at"</span><span class="token punctuation">:</span> <span class="token string">"2021-05-10T09:59:36.000000Z"</span><span class="token punctuation">,</span>
    <span class="token string">"is_unread"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"from_uid"</span><span class="token punctuation">:</span> <span class="token string">"2671362"</span><span class="token punctuation">,</span>
    <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"4"</span><span class="token punctuation">,</span>
    <span class="token string">"notification_key"</span><span class="token punctuation">:</span> <span class="token string">"notification_123"</span><span class="token punctuation">,</span>
    <span class="token string">"notification_type"</span><span class="token punctuation">:</span> <span class="token string">"user_left_project"</span><span class="token punctuation">,</span>
    <span class="token string">"project_id"</span><span class="token punctuation">:</span> <span class="token string">"6Jf8VQXxpwv56VQ7"</span><span class="token punctuation">,</span>
    <span class="token string">"seq_no"</span><span class="token punctuation">:</span> <span class="token number">1234567890</span>
<span class="token punctuation">}</span>

<span class="token punctuation">{</span>
    <span class="token string">"created_at"</span><span class="token punctuation">:</span> <span class="token string">"2021-05-10T09:59:36.000000Z"</span><span class="token punctuation">,</span>
    <span class="token string">"is_unread"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"from_uid"</span><span class="token punctuation">:</span> <span class="token string">"2671362"</span><span class="token punctuation">,</span>
    <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"5"</span><span class="token punctuation">,</span>
    <span class="token string">"notification_key"</span><span class="token punctuation">:</span> <span class="token string">"notification_123"</span><span class="token punctuation">,</span>
    <span class="token string">"notification_type"</span><span class="token punctuation">:</span> <span class="token string">"user_removed_from_project"</span><span class="token punctuation">,</span>
    <span class="token string">"project_id"</span><span class="token punctuation">:</span> <span class="token string">"6Jf8VQXxpwv56VQ7"</span><span class="token punctuation">,</span>
    <span class="token string">"removed_name"</span><span class="token punctuation">:</span> <span class="token string">"Example User"</span><span class="token punctuation">,</span>
    <span class="token string">"removed_uid"</span><span class="token punctuation">:</span> <span class="token string">"2671366"</span><span class="token punctuation">,</span>
    <span class="token string">"seq_no"</span><span class="token punctuation">:</span> <span class="token number">1234567890</span>
<span class="token punctuation">}</span>

<span class="token punctuation">{</span>
    <span class="token string">"assigned_by_uid"</span><span class="token punctuation">:</span> <span class="token string">"2671362"</span><span class="token punctuation">,</span>
    <span class="token string">"created_at"</span><span class="token punctuation">:</span> <span class="token string">"2021-05-10T09:59:36.000000Z"</span><span class="token punctuation">,</span>
    <span class="token string">"is_unread"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"from_uid"</span><span class="token punctuation">:</span> <span class="token string">"2671362"</span><span class="token punctuation">,</span>
    <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"6"</span><span class="token punctuation">,</span>
    <span class="token string">"item_content"</span><span class="token punctuation">:</span> <span class="token string">"NewTask"</span><span class="token punctuation">,</span>
    <span class="token string">"item_id"</span><span class="token punctuation">:</span> <span class="token string">"6X7gfV9G7rWm5hW8"</span><span class="token punctuation">,</span>
    <span class="token string">"notification_key"</span><span class="token punctuation">:</span> <span class="token string">"notification_123"</span><span class="token punctuation">,</span>
    <span class="token string">"notification_type"</span><span class="token punctuation">:</span> <span class="token string">"item_assigned"</span><span class="token punctuation">,</span>
    <span class="token string">"project_id"</span><span class="token punctuation">:</span> <span class="token string">"6Jf8VQXxpwv56VQ7"</span><span class="token punctuation">,</span>
    <span class="token string">"responsible_uid"</span><span class="token punctuation">:</span> <span class="token string">"2671355"</span><span class="token punctuation">,</span>
    <span class="token string">"seq_no"</span><span class="token punctuation">:</span> <span class="token number">1234567890</span>
<span class="token punctuation">}</span>

<span class="token punctuation">{</span>
    <span class="token string">"assigned_by_uid"</span><span class="token punctuation">:</span> <span class="token string">"2671362"</span><span class="token punctuation">,</span>
    <span class="token string">"created_at"</span><span class="token punctuation">:</span> <span class="token string">"2021-05-10T09:59:36.000000Z"</span><span class="token punctuation">,</span>
    <span class="token string">"is_unread"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"from_uid"</span><span class="token punctuation">:</span> <span class="token string">"2671362"</span><span class="token punctuation">,</span>
    <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"7"</span><span class="token punctuation">,</span>
    <span class="token string">"item_content"</span><span class="token punctuation">:</span> <span class="token string">"NewTask"</span><span class="token punctuation">,</span>
    <span class="token string">"item_id"</span><span class="token punctuation">:</span> <span class="token string">"6X7gfV9G7rWm5hW8"</span><span class="token punctuation">,</span>
    <span class="token string">"notification_key"</span><span class="token punctuation">:</span> <span class="token string">"notification_123"</span><span class="token punctuation">,</span>
    <span class="token string">"notification_type"</span><span class="token punctuation">:</span> <span class="token string">"item_completed"</span><span class="token punctuation">,</span>
    <span class="token string">"project_id"</span><span class="token punctuation">:</span> <span class="token string">"6Jf8VQXxpwv56VQ7"</span><span class="token punctuation">,</span>
    <span class="token string">"responsible_uid"</span><span class="token punctuation">:</span> <span class="token string">"2671355"</span><span class="token punctuation">,</span>
    <span class="token string">"seq_no"</span><span class="token punctuation">:</span> <span class="token number">1234567890</span>
<span class="token punctuation">}</span>

<span class="token punctuation">{</span>
    <span class="token string">"assigned_by_uid"</span><span class="token punctuation">:</span> <span class="token string">"2671362"</span><span class="token punctuation">,</span>
    <span class="token string">"created_at"</span><span class="token punctuation">:</span> <span class="token string">"2021-05-10T09:59:36.000000Z"</span><span class="token punctuation">,</span>
    <span class="token string">"is_unread"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"from_uid"</span><span class="token punctuation">:</span> <span class="token string">"2671362"</span><span class="token punctuation">,</span>
    <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"8"</span><span class="token punctuation">,</span>
    <span class="token string">"item_id"</span><span class="token punctuation">:</span> <span class="token string">"6X7gfV9G7rWm5hW8"</span><span class="token punctuation">,</span>
    <span class="token string">"item_content"</span><span class="token punctuation">:</span> <span class="token string">"NewTask"</span><span class="token punctuation">,</span>
    <span class="token string">"notification_key"</span><span class="token punctuation">:</span> <span class="token string">"notification_123"</span><span class="token punctuation">,</span>
    <span class="token string">"notification_type"</span><span class="token punctuation">:</span> <span class="token string">"item_uncompleted"</span><span class="token punctuation">,</span>
    <span class="token string">"project_id"</span><span class="token punctuation">:</span> <span class="token string">"6Jf8VQXxpwv56VQ7"</span><span class="token punctuation">,</span>
    <span class="token string">"responsible_uid"</span><span class="token punctuation">:</span> <span class="token string">"321"</span><span class="token punctuation">,</span>
    <span class="token string">"seq_no"</span><span class="token punctuation">:</span> <span class="token number">1234567890</span>
<span class="token punctuation">}</span>

<span class="token punctuation">{</span>
    <span class="token string">"created_at"</span><span class="token punctuation">:</span> <span class="token string">"2021-05-10T09:59:36.000000Z"</span><span class="token punctuation">,</span>
    <span class="token string">"is_unread"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"from_uid"</span><span class="token punctuation">:</span> <span class="token string">"2671362"</span><span class="token punctuation">,</span>
    <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"9"</span><span class="token punctuation">,</span>
    <span class="token string">"item_id"</span><span class="token punctuation">:</span> <span class="token string">"6X7gfV9G7rWm5hW8"</span><span class="token punctuation">,</span>
    <span class="token string">"note_content"</span><span class="token punctuation">:</span> <span class="token string">"NewTask"</span><span class="token punctuation">,</span>
    <span class="token string">"note_id"</span><span class="token punctuation">:</span> <span class="token string">"6X7jp7j8x7JhWFC3"</span><span class="token punctuation">,</span>
    <span class="token string">"notification_key"</span><span class="token punctuation">:</span> <span class="token string">"notification_123"</span><span class="token punctuation">,</span>
    <span class="token string">"notification_type"</span><span class="token punctuation">:</span> <span class="token string">"note_added"</span><span class="token punctuation">,</span>
    <span class="token string">"project_id"</span><span class="token punctuation">:</span> <span class="token string">"6Jf8VQXxpwv56VQ7"</span><span class="token punctuation">,</span>
    <span class="token string">"seq_no"</span><span class="token punctuation">:</span> <span class="token number">1234567890</span>
<span class="token punctuation">}</span>

<span class="token punctuation">{</span>
    <span class="token string">"created_at"</span><span class="token punctuation">:</span> <span class="token string">"2021-05-10T09:59:36.000000Z"</span><span class="token punctuation">,</span>
    <span class="token string">"is_unread"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"from_uid"</span><span class="token punctuation">:</span> <span class="token string">"2671362"</span><span class="token punctuation">,</span>
    <span class="token string">"count"</span><span class="token punctuation">:</span> <span class="token number">5</span><span class="token punctuation">,</span>
    <span class="token string">"goal"</span><span class="token punctuation">:</span> <span class="token number">5</span><span class="token punctuation">,</span>
    <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"18"</span><span class="token punctuation">,</span>
    <span class="token string">"notification_key"</span><span class="token punctuation">:</span> <span class="token string">"notification_123"</span><span class="token punctuation">,</span>
    <span class="token string">"notification_type"</span><span class="token punctuation">:</span> <span class="token string">"daily_goal_reached"</span><span class="token punctuation">,</span>
    <span class="token string">"seq_no"</span><span class="token punctuation">:</span> <span class="token number">1234567890</span>
<span class="token punctuation">}</span>

<span class="token punctuation">{</span>
    <span class="token string">"created_at"</span><span class="token punctuation">:</span> <span class="token string">"2021-05-10T09:59:36.000000Z"</span><span class="token punctuation">,</span>
    <span class="token string">"is_unread"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"from_uid"</span><span class="token punctuation">:</span> <span class="token string">"2671362"</span><span class="token punctuation">,</span>
    <span class="token string">"count"</span><span class="token punctuation">:</span> <span class="token number">50</span><span class="token punctuation">,</span>
    <span class="token string">"goal"</span><span class="token punctuation">:</span> <span class="token number">50</span><span class="token punctuation">,</span>
    <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"19"</span><span class="token punctuation">,</span>
    <span class="token string">"notification_key"</span><span class="token punctuation">:</span> <span class="token string">"notification_123"</span><span class="token punctuation">,</span>
    <span class="token string">"notification_type"</span><span class="token punctuation">:</span> <span class="token string">"weekly_goal_reached"</span><span class="token punctuation">,</span>
    <span class="token string">"seq_no"</span><span class="token punctuation">:</span> <span class="token number">1234567890</span>
<span class="token punctuation">}</span>
</code></pre>
<h4 id="types">Types</h4>
<p>This is the list of notifications which can be issued by the system:</p>
<table>
<thead>
<tr>
<th>Type</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>share_invitation_sent</td>
<td>Sent to the sharing invitation receiver.</td>
</tr>
<tr>
<td>share_invitation_accepted</td>
<td>Sent to the sharing invitation sender, when the receiver accepts the invitation.</td>
</tr>
<tr>
<td>share_invitation_rejected</td>
<td>Sent to the sharing invitation sender, when the receiver rejects the invitation.</td>
</tr>
<tr>
<td>user_left_project</td>
<td>Sent to everyone when somebody leaves the project.</td>
</tr>
<tr>
<td>user_removed_from_project</td>
<td>Sent to everyone, when a person removes somebody from the project.</td>
</tr>
<tr>
<td>item_assigned</td>
<td>Sent to user who is responsible for the task. Optionally it&#39;s also sent to the user who created the task initially, if the assigner and the task creator is not the same person.</td>
</tr>
<tr>
<td>item_completed</td>
<td>Sent to the user who assigned the task when the task is completed. Optionally it&#39;s also sent to the user who is responsible for this task, if the responsible user and the user who completed the task is not the same person.</td>
</tr>
<tr>
<td>item_uncompleted</td>
<td>Sent to the user who assigned the task when the task is uncompleted. Optionally it&#39;s also sent to the user who is responsible for this task, if the responsible user and the user who completed the task is not the same person.</td>
</tr>
<tr>
<td>note_added</td>
<td>Sent to all members of the shared project, whenever someone adds a note to the task.</td>
</tr>
<tr>
<td>workspace_invitation_created</td>
<td>Sent to the invitee (if existing user) when invited to a workspace.</td>
</tr>
<tr>
<td>workspace_invitation_accepted</td>
<td>Sent to the inviter, and admins of paid workspaces, when the workspace invitation is accepted.</td>
</tr>
<tr>
<td>workspace_invitation_rejected</td>
<td>Sent to the inviter when the workspace invitation is declined.</td>
</tr>
<tr>
<td>project_archived</td>
<td>Sent to project collaborators when the project is archived. <em>Only for workspace projects at the moment.</em></td>
</tr>
<tr>
<td>removed_from_workspace</td>
<td>Sent to removed user when removed from a workspace.</td>
</tr>
<tr>
<td>workspace_deleted</td>
<td>Sent to every workspace admin, member and guest.</td>
</tr>
<tr>
<td>teams_workspace_upgraded</td>
<td>Sent to workspace admins and members when workspace is upgraded to paid plan (access to paid features).</td>
</tr>
<tr>
<td>teams_workspace_canceled</td>
<td>Sent to workspace admins and members when workspace is back on Starter plan (no access to paid features).</td>
</tr>
<tr>
<td>teams_workspace_payment_failed</td>
<td>Sent to the workspace billing admin on the web when a payment failed since it requires their action.</td>
</tr>
<tr>
<td>karma_level</td>
<td>Sent when a new karma level is reached</td>
</tr>
<tr>
<td>share_invitation_blocked_by_project_limit</td>
<td>Sent when the invitation is blocked because the user reached the project limits</td>
</tr>
<tr>
<td>workspace_user_joined_by_domain</td>
<td>Sent when a user join a new workspace by domain</td>
</tr>
</tbody></table>
<h4 id="common-properties">Common properties</h4>
<p>Some properties are common for all types of notifications, whereas some others
depend on the notification type.</p>
<p>Every live notification has the following properties:</p>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>The ID of the live notification.</td>
</tr>
<tr>
<td>created_at <em>String</em></td>
<td>Live notification creation date.</td>
</tr>
<tr>
<td>from_uid <em>String</em></td>
<td>The ID of the user who initiated this live notification.</td>
</tr>
<tr>
<td>notification_key <em>String</em></td>
<td>Unique notification key.</td>
</tr>
<tr>
<td>notification_type <em>String</em></td>
<td>Type of notification. Different notification type define different extra fields which are described below.</td>
</tr>
<tr>
<td>seq_no <em>Integer</em></td>
<td>Notification sequence number.</td>
</tr>
<tr>
<td>is_unread <em>Boolean</em></td>
<td>Whether the notification is marked as unread (a <code>true</code> or <code>false</code> value).</td>
</tr>
</tbody></table>
<h4 id="specific-properties">Specific properties</h4>
<p>Here are the extra properties for the <code>*_invitation_*</code> types of live
notifications:</p>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>from_user <em>Object</em></td>
<td>User data, useful on <code>share_invitation_sent</code>.</td>
</tr>
<tr>
<td>project_name <em>String</em></td>
<td>The project name, useful for <code>share_invitation_*</code> where you may not have the project in the local model.</td>
</tr>
<tr>
<td>invitation_id <em>String</em></td>
<td>The invitation ID. Useful for accepting/rejecting invitations.</td>
</tr>
<tr>
<td>invitation_secret <em>String</em></td>
<td>The invitation secret key. Useful for accepting/rejecting invitations.</td>
</tr>
</tbody></table>
<p>Here are the extra properties for the <code>share_invitation_sent</code> type of live notifications:</p>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>state <em>String</em></td>
<td>Invitation state. Initially <code>invited</code>, can change the state to <code>accepted</code> or <code>rejected</code>.</td>
</tr>
</tbody></table>
<p>Here are the extra properties for the <code>user_removed_from_project</code> type of live notifications:</p>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>removed_name <em>String</em></td>
<td>The name of the user removed.</td>
</tr>
<tr>
<td>removed_uid <em>String</em></td>
<td>The uid of the user removed.</td>
</tr>
</tbody></table>
<p>Here are the extra properties for the <code>workspace_invitation_created</code> types of live
notifications:</p>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>from_user <em>Object</em></td>
<td>User data, same as in <code>share_invitation_sent</code>.</td>
</tr>
<tr>
<td>workspace_id <em>Integer</em></td>
<td>The ID of the workspace.</td>
</tr>
<tr>
<td>workspace_name <em>String</em></td>
<td>Name of the workspace.</td>
</tr>
<tr>
<td>invitation_id <em>String</em></td>
<td>The invitation ID. Useful for accepting/rejecting invitations.</td>
</tr>
<tr>
<td>invitation_secret <em>String</em></td>
<td>Invitation secret. Should be used to accept or reject invitation.</td>
</tr>
<tr>
<td>state <em>String</em></td>
<td>Invitation state. Initially <code>invited</code>, can change the state to <code>accepted</code> or <code>rejected</code>.</td>
</tr>
</tbody></table>
<p>Here are the extra properties for the <code>workspace_invitation_accepted</code> and <code>workspace_invitation_rejected</code> types of live
notifications:</p>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>from_user <em>Object</em></td>
<td>User data, same as in <code>share_invitation_sent</code>.</td>
</tr>
<tr>
<td>workspace_id <em>Integer</em></td>
<td>The ID of the workspace.</td>
</tr>
<tr>
<td>workspace_name <em>String</em></td>
<td>Name of the workspace.</td>
</tr>
<tr>
<td>invitation_id <em>String</em></td>
<td>The invitation ID. Useful for accepting/rejecting invitations.</td>
</tr>
</tbody></table>
<p>Here are the extra properties for the <code>removed_from_workspace</code> and <code>workspace_deleted</code> types of live
notifications:</p>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>from_user <em>Object</em></td>
<td>User data, same as in <code>share_invitation_sent</code>.</td>
</tr>
<tr>
<td>workspace_id <em>Integer</em></td>
<td>The ID of the workspace.</td>
</tr>
<tr>
<td>workspace_name <em>String</em></td>
<td>Name of the workspace.</td>
</tr>
</tbody></table>
<p>Here are the extra properties for the <code>teams_workspace_upgraded</code>, <code>teams_workspace_canceled</code> and <code>teams_workspace_payment_failed</code> types of live notifications:</p>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>workspace_id <em>Integer</em></td>
<td>The ID of the workspace.</td>
</tr>
<tr>
<td>workspace_name <em>String</em></td>
<td>Name of the workspace.</td>
</tr>
<tr>
<td>plan_type <em>String</em></td>
<td>Tariff plan name for the workspace. Valid values are <code>STARTER</code> and <code>BUSINESS</code>.</td>
</tr>
</tbody></table>
<p>Here are the extra properties for the <code>project_archived</code> types of live
notifications:</p>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>from_user <em>Object</em></td>
<td>User data, same as in <code>share_invitation_sent</code>.</td>
</tr>
<tr>
<td>project_id <em>Integer</em></td>
<td>The ID of the project.</td>
</tr>
<tr>
<td>project_name <em>String</em></td>
<td>Name of the project.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example set last known notification request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "live_notifications_set_last_read",
        "uuid": "588b9ccf-29c0-4837-8bbc-fc858c0c6df8",
        "args": {"id": "1234"}
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"588b9ccf-29c0-4837-8bbc-fc858c0c6df8"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Set the last known notification.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>The ID of the last known notification (a number or <code>0</code> or <code>null</code> to mark all read).</td>
</tr>
</tbody></table>

<blockquote>
<p>Example mark notification read request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "live_notifications_mark_read",
        "uuid": "588b9ccf-29c0-4837-8bbc-fc858c0c6df8",
        "args": {"ids": ["1234"]}
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"588b9ccf-29c0-4837-8bbc-fc858c0c6df8"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Mark the notifications as read.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>ids <em>Array of String</em></td>
<td>Yes</td>
<td>The IDs of the notifications.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example mark all notifications read request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "live_notifications_mark_read_all",
        "uuid": "588b9ccf-29c0-4837-8bbc-fc858c0c6df8"
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"588b9ccf-29c0-4837-8bbc-fc858c0c6df8"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Mark all notifications as read.</p>

<blockquote>
<p>Example mark notification unread request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "live_notifications_mark_unread",
        "uuid": "588b9ccf-29c0-4837-8bbc-fc858c0c6df8",
        "args": {"ids": ["1234"]}
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"588b9ccf-29c0-4837-8bbc-fc858c0c6df8"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Mark the notifications as unread.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>ids <em>Array of String</em></td>
<td>Yes</td>
<td>The IDs of the notifications.</td>
</tr>
</tbody></table>

<blockquote>
<p>An example personal label object:</p>
</blockquote>
<pre><code><span class="token punctuation">{</span>
    <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"2156154810"</span><span class="token punctuation">,</span>
    <span class="token string">"name"</span><span class="token punctuation">:</span> <span class="token string">"Food"</span><span class="token punctuation">,</span>
    <span class="token string">"color"</span><span class="token punctuation">:</span> <span class="token string">"lime_green"</span><span class="token punctuation">,</span>
    <span class="token string">"item_order"</span><span class="token punctuation">:</span> <span class="token number">0</span><span class="token punctuation">,</span>
    <span class="token string">"is_deleted"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"is_favorite"</span><span class="token punctuation">:</span> <span class="token boolean">false</span>
<span class="token punctuation">}</span>
</code></pre>
<p>There are two types of labels that can be added to Todoist tasks.
We refer to these as <code>personal</code> and <code>shared</code> labels.</p>
<h4 id="personal-labels">Personal labels</h4>
<p>Labels created by the current user will show up in their personal label list.
These labels can be customized and will stay in their account unless deleted.</p>
<p>A personal label can be converted to a shared label by the user if they no longer
require them to be stored against their account, but they still appear on
shared tasks.</p>
<h4 id="shared-labels">Shared labels</h4>
<p>A label created by a collaborator that doesn&#39;t share a name with an existing personal label
will appear in our clients as a shared label. These labels are gray by default and will
only stay in the shared labels list if there are any active tasks with this label.</p>
<p>A user can convert a shared label to a personal label at any time. The label will then become
customizable and will remain in the account even if not assigned to any active tasks.</p>
<p>Shared labels do not appear in the sync response for a user&#39;s account. They only appear
within the <code>labels</code> list of the <a href="#tag/Sync/Tasks">tasks</a> that they are assigned to.</p>
<p>You can find more information on the differences between personal and shared labels in our <a href="https://www.todoist.com/help/articles/introduction-to-labels-dSo2eE#shared">Help Center</a>.</p>
<h4 id="properties-only-applicable-to-personal-labels">Properties (only applicable to personal labels)</h4>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>The ID of the label.</td>
</tr>
<tr>
<td>name <em>String</em></td>
<td>The name of the label.</td>
</tr>
<tr>
<td>color <em>String</em></td>
<td>The color of the label icon. Refer to the <code>name</code> column in the <a href="#tag/Colors">Colors</a> guide for more info.</td>
</tr>
<tr>
<td>item_order <em>Integer</em></td>
<td>Label’s order in the label list (a number, where the smallest value should place the label at the top).</td>
</tr>
<tr>
<td>is_deleted <em>Boolean</em></td>
<td>Whether the label is marked as deleted (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>is_favorite <em>Boolean</em></td>
<td>Whether the label is a favorite (a <code>true</code> or <code>false</code> value).</td>
</tr>
</tbody></table>

<blockquote>
<p>Example add label request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "label_add",
        "temp_id": "f2f182ed-89fa-4bbb-8a42-ec6f7aa47fd0",
        "uuid": "ba204343-03a4-41ff-b964-95a102d12b35",
        "args": {"name": "Food"}
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"ba204343-03a4-41ff-b964-95a102d12b35"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token string">"temp_id_mapping"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"f2f182ed-89fa-4bbb-8a42-ec6f7aa47fd0"</span><span class="token builtin class-name">:</span> <span class="token string">"2156154810"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>name <em>String</em></td>
<td>Yes</td>
<td>The name of the label</td>
</tr>
<tr>
<td>color <em>String</em></td>
<td>No</td>
<td>The color of the label icon. Refer to the <code>name</code> column in the <a href="#tag/Colors">Colors</a> guide for more info.</td>
</tr>
<tr>
<td>item_order <em>Integer</em></td>
<td>No</td>
<td>Label’s order in the label list (a number, where the smallest value should place the label at the top).</td>
</tr>
<tr>
<td>is_favorite <em>Boolean</em></td>
<td>No</td>
<td>Whether the label is a favorite (a <code>true</code> or <code>false</code> value).</td>
</tr>
</tbody></table>

<blockquote>
<p>Example update label request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "label_update",
        "uuid": "9c9a6e34-2382-4f43-a217-9ab017a83523",
        "args": {"id": "2156154810", "color": "berry_red"}
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"9c9a6e34-2382-4f43-a217-9ab017a83523"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>The ID of the label.</td>
</tr>
<tr>
<td>name <em>String</em></td>
<td>No</td>
<td>The name of the label.</td>
</tr>
<tr>
<td>color <em>String</em></td>
<td>No</td>
<td>The color of the label icon. Refer to the <code>name</code> column in the <a href="#tag/Colors">Colors</a> guide for more info.</td>
</tr>
<tr>
<td>item_order <em>Integer</em></td>
<td>No</td>
<td>Label’s order in the label list.</td>
</tr>
<tr>
<td>is_favorite <em>Boolean</em></td>
<td>No</td>
<td>Whether the label is a favorite (a <code>true</code> or <code>false</code> value).</td>
</tr>
</tbody></table>

<blockquote>
<p>Example delete label request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "label_delete",
        "uuid": "aabaa5e0-b91b-439c-aa83-d1b35a5e9fb3",
        "args": {
            "id": "2156154810",
            "cascade": "all"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"aabaa5e0-b91b-439c-aa83-d1b35a5e9fb3"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>The ID of the label.</td>
</tr>
<tr>
<td>cascade <em>String</em></td>
<td>No</td>
<td>A string value, either <code>all</code> (default) or <code>none</code>. If no value or <code>all</code> is passed, the personal label will be removed and any instances of the label will also be removed from tasks (including tasks in shared projects). If <code>none</code> is passed, the personal label will be removed from the user&#39;s account but it will continue to appear on tasks as a shared label.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example rename shared label request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "label_rename",
        "uuid": "b863b0e5-2541-4a5a-a462-ce265ae2ff2d",
        "args": {
            "name_old": "Food",
            "name_new": "Drink"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"b863b0e5-2541-4a5a-a462-ce265ae2ff2d"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>This command enables renaming of shared labels. Any tasks containing a label matching the
value of <code>name_old</code> will be updated with the new label name.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>name_old <em>String</em></td>
<td>Yes</td>
<td>The current name of the label to modify.</td>
</tr>
<tr>
<td>name_new <em>String</em></td>
<td>Yes</td>
<td>The new name for the label.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example delete shared label request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "label_delete_occurrences",
        "uuid": "6174264a-2842-410c-a8ff-603ec4d4736b",
        "args": {
            "name": "Shopping"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"6174264a-2842-410c-a8ff-603ec4d4736b"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Deletes all occurrences of a shared label from any active tasks.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>name <em>String</em></td>
<td>Yes</td>
<td>The name of the label to remove.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example update label orders request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token punctuation">[</span>
    <span class="token punctuation">{</span>
        <span class="token string">"type"</span><span class="token builtin class-name">:</span> <span class="token string">"label_update_orders"</span>,
        <span class="token string">"uuid"</span><span class="token builtin class-name">:</span> <span class="token string">"1402a911-5b7a-4beb-bb1f-fb9e1ed798fb"</span>,
        <span class="token string">"args"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
            <span class="token string">"id_order_mapping"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"2156154810"</span><span class="token builtin class-name">:</span>  <span class="token number">1</span>, <span class="token string">"2156154820"</span><span class="token builtin class-name">:</span> <span class="token number">2</span><span class="token punctuation">}</span>
        <span class="token punctuation">}</span>
    <span class="token punctuation">}</span><span class="token punctuation">]</span>'
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
    <span class="token string">"517560cc-f165-4ff6-947b-3adda8aef744"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
    <span class="token punctuation">..</span>.
  <span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id_order_mapping <em>Object</em></td>
<td>Yes</td>
<td>A dictionary, where a label <code>id</code> is the key, and the <code>item_order</code> value.</td>
</tr>
</tbody></table>

<blockquote>
<p>An example task object:</p>
</blockquote>
<pre><code><span class="token punctuation">{</span>
    <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"6X7rM8997g3RQmvh"</span><span class="token punctuation">,</span>
    <span class="token string">"user_id"</span><span class="token punctuation">:</span> <span class="token string">"2671355"</span><span class="token punctuation">,</span>
    <span class="token string">"project_id"</span><span class="token punctuation">:</span> <span class="token string">"6Jf8VQXxpwv56VQ7"</span><span class="token punctuation">,</span>
    <span class="token string">"content"</span><span class="token punctuation">:</span> <span class="token string">"Buy Milk"</span><span class="token punctuation">,</span>
    <span class="token string">"description"</span><span class="token punctuation">:</span> <span class="token string">""</span><span class="token punctuation">,</span>
    <span class="token string">"priority"</span><span class="token punctuation">:</span> <span class="token number">1</span><span class="token punctuation">,</span>
    <span class="token string">"due"</span><span class="token punctuation">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
    <span class="token string">"deadline"</span><span class="token punctuation">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
    <span class="token string">"parent_id"</span><span class="token punctuation">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
    <span class="token string">"child_order"</span><span class="token punctuation">:</span> <span class="token number">1</span><span class="token punctuation">,</span>
    <span class="token string">"section_id"</span><span class="token punctuation">:</span> <span class="token string">"3Ty8VQXxpwv28PK3"</span><span class="token punctuation">,</span>
    <span class="token string">"day_order"</span><span class="token punctuation">:</span> <span class="token operator">-</span><span class="token number">1</span><span class="token punctuation">,</span>
    <span class="token string">"is_collapsed"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"labels"</span><span class="token punctuation">:</span> <span class="token punctuation">[</span><span class="token string">"Food"</span><span class="token punctuation">,</span> <span class="token string">"Shopping"</span><span class="token punctuation">]</span><span class="token punctuation">,</span>
    <span class="token string">"added_by_uid"</span><span class="token punctuation">:</span> <span class="token string">"2671355"</span><span class="token punctuation">,</span>
    <span class="token string">"assigned_by_uid"</span><span class="token punctuation">:</span> <span class="token string">"2671355"</span><span class="token punctuation">,</span>
    <span class="token string">"responsible_uid"</span><span class="token punctuation">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
    <span class="token string">"checked"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"is_deleted"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"added_at"</span><span class="token punctuation">:</span> <span class="token string">"2025-01-21T21:28:43.841504Z"</span><span class="token punctuation">,</span>
    <span class="token string">"updated_at"</span><span class="token punctuation">:</span> <span class="token string">"2025-01-21T21:28:43Z"</span><span class="token punctuation">,</span>
    <span class="token string">"completed_at"</span><span class="token punctuation">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
    <span class="token string">"deadline"</span><span class="token punctuation">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
    <span class="token string">"duration"</span><span class="token punctuation">:</span> <span class="token punctuation">{</span>
        <span class="token string">"amount"</span><span class="token punctuation">:</span> <span class="token number">15</span><span class="token punctuation">,</span>
        <span class="token string">"unit"</span><span class="token punctuation">:</span> <span class="token string">"minute"</span>
    <span class="token punctuation">}</span>
</code></pre>
<h4 id="properties">Properties</h4>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>The ID of the task.</td>
</tr>
<tr>
<td>user_id <em>String</em></td>
<td>The owner of the task.</td>
</tr>
<tr>
<td>project_id <em>String</em></td>
<td>The ID of the parent project.</td>
</tr>
<tr>
<td>content <em>String</em></td>
<td>The text of the task. This value may contain markdown-formatted text and hyperlinks. Details on markdown support can be found in the <a href="https://www.todoist.com/help/articles/format-text-in-a-todoist-task-e5dHw9">Text Formatting article</a> in the Help Center.</td>
</tr>
<tr>
<td>description <em>String</em></td>
<td>A description for the task. This value may contain markdown-formatted text and hyperlinks. Details on markdown support can be found in the <a href="https://www.todoist.com/help/articles/format-text-in-a-todoist-task-e5dHw9">Text Formatting article</a> in the Help Center.</td>
</tr>
<tr>
<td>due <em>Object</em></td>
<td>The due date of the task. See the <a href="#tag/Due-dates">Due dates</a> section for more details.</td>
</tr>
<tr>
<td>deadline <em>Object</em></td>
<td>The deadline of the task. See the <a href="#tag/Deadlines">Deadlines</a> section for more details.</td>
</tr>
<tr>
<td>priority <em>Integer</em></td>
<td>The priority of the task (a number between <code>1</code> and <code>4</code>, <code>4</code> for very urgent and <code>1</code> for natural). <br><strong>Note</strong>: Keep in mind that <code>very urgent</code> is the priority 1 on clients. So, <code>p1</code> will return <code>4</code> in the API.</td>
</tr>
<tr>
<td>parent_id <em>String</em></td>
<td>The ID of the parent task. Set to <code>null</code> for root tasks.</td>
</tr>
<tr>
<td>child_order <em>Integer</em></td>
<td>The order of the task. Defines the position of the task among all the tasks with the same parent.</td>
</tr>
<tr>
<td>section_id <em>String</em></td>
<td>The ID of the parent section. Set to <code>null</code> for tasks not belonging to a section.</td>
</tr>
<tr>
<td>day_order <em>Integer</em></td>
<td>The order of the task inside the <code>Today</code> or <code>Next 7 days</code> view (a number, where the smallest value would place the task at the top).</td>
</tr>
<tr>
<td>is_collapsed <em>Boolean</em></td>
<td>Whether the task&#39;s sub-tasks are collapsed (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>labels <em>Array of String</em></td>
<td>The task&#39;s labels (a list of names that may represent either personal or shared labels).</td>
</tr>
<tr>
<td>added_by_uid <em>String</em></td>
<td>The ID of the user who created the task. This makes sense for shared projects only. For tasks created before 31 Oct 2019 the value is set to null. Cannot be set explicitly or changed via API.</td>
</tr>
<tr>
<td>assigned_by_uid <em>String</em></td>
<td>The ID of the user who assigned the task. This makes sense for shared projects only. Accepts any user ID from the list of project collaborators. If this value is unset or invalid, it will automatically be set up to your uid.</td>
</tr>
<tr>
<td>responsible_uid <em>String</em></td>
<td>The ID of user who is responsible for accomplishing the current task. This makes sense for shared projects only. Accepts any user ID from the list of project collaborators or <code>null</code> or an empty string to unset.</td>
</tr>
<tr>
<td>checked <em>Boolean</em></td>
<td>Whether the task is marked as completed (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>is_deleted <em>Boolean</em></td>
<td>Whether the task is marked as deleted (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>completed_at <em>String</em></td>
<td>The date when the task was completed (or <code>null</code> if not completed).</td>
</tr>
<tr>
<td>added_at <em>String</em></td>
<td>The datetime when the task was created.</td>
</tr>
<tr>
<td>updated_at <em>String</em></td>
<td>The datetime when the task was updated.</td>
</tr>
<tr>
<td>completed_at <em>String</em></td>
<td>The datetime when the task was completed.</td>
</tr>
<tr>
<td>duration <em>Object</em></td>
<td>Object representing a task&#39;s duration. Includes a positive integer (greater than zero) for the <code>amount</code> of time the task will take, and the <code>unit</code> of time that the amount represents which must be either <code>minute</code> or <code>day</code>. Both the <code>amount</code> and <code>unit</code> <strong>must</strong> be defined. The object will be <code>null</code> if the task has no duration.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example add task request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "item_add",
        "temp_id": "43f7ed23-a038-46b5-b2c9-4abda9097ffa",
        "uuid": "997d4b43-55f1-48a9-9e66-de5785dfd69b",
        "args": {
            "content": "Buy Milk",
            "project_id": "6Jf8VQXxpwv56VQ7",
            "labels": ["Food", "Shopping"]
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"997d4b43-55f1-48a9-9e66-de5785dfd69b"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token string">"temp_id_mapping"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"43f7ed23-a038-46b5-b2c9-4abda9097ffa"</span><span class="token builtin class-name">:</span> <span class="token string">"6X7rM8997g3RQmvh"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Add a new task to a project.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>content <em>String</em></td>
<td>Yes</td>
<td>The text of the task. This value may contain markdown-formatted text and hyperlinks. Details on markdown support can be found in the <a href="https://www.todoist.com/help/articles/format-text-in-a-todoist-task-e5dHw9">Text Formatting article</a> in the Help Center.</td>
</tr>
<tr>
<td>description <em>String</em></td>
<td>No</td>
<td>A description for the task. This value may contain markdown-formatted text and hyperlinks. Details on markdown support can be found in the <a href="https://www.todoist.com/help/articles/format-text-in-a-todoist-task-e5dHw9">Text Formatting article</a> in the Help Center.</td>
</tr>
<tr>
<td>project_id <em>String</em></td>
<td>No</td>
<td>The ID of the project to add the task to (a number or a temp id). By default the task is added to the user’s <code>Inbox</code> project.</td>
</tr>
<tr>
<td>due <em>Object</em></td>
<td>No</td>
<td>The due date of the task. See the <a href="#tag/Due-dates">Due dates</a> section for more details.</td>
</tr>
<tr>
<td>deadline <em>Object</em></td>
<td>No</td>
<td>The deadline of the task. See the <a href="#tag/Deadlines">Deadlines</a> section for more details.</td>
</tr>
<tr>
<td>priority <em>Integer</em></td>
<td>No</td>
<td>The priority of the task (a number between <code>1</code> and <code>4</code>, <code>4</code> for very urgent and <code>1</code> for natural). <br><strong>Note</strong>: Keep in mind that <code>very urgent</code> is the priority 1 on clients. So, <code>p1</code> will return <code>4</code> in the API.</td>
</tr>
<tr>
<td>parent_id <em>String</em></td>
<td>No</td>
<td>The ID of the parent task. Set to <code>null</code> for root tasks.</td>
</tr>
<tr>
<td>child_order <em>Integer</em></td>
<td>No</td>
<td>The order of task. Defines the position of the task among all the tasks with the same parent.</td>
</tr>
<tr>
<td>section_id <em>String</em></td>
<td>No</td>
<td>The ID of the section. Set to <code>null</code> for tasks not belonging to a section.</td>
</tr>
<tr>
<td>day_order <em>Integer</em></td>
<td>No</td>
<td>The order of the task inside the <code>Today</code> or <code>Next 7 days</code> view (a number, where the smallest value would place the task at the top).</td>
</tr>
<tr>
<td>is_collapsed <em>Boolean</em></td>
<td>No</td>
<td>Whether the task&#39;s sub-tasks are collapsed (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>labels <em>Array of String</em></td>
<td>No</td>
<td>The task&#39;s labels (a list of names that may represent either personal or shared labels).</td>
</tr>
<tr>
<td>assigned_by_uid <em>String</em></td>
<td>No</td>
<td>The ID of user who assigns the current task. This makes sense for shared projects only. Accepts <code>0</code> or any user ID from the list of project collaborators. If this value is unset or invalid, it will be automatically setup to your uid.</td>
</tr>
<tr>
<td>responsible_uid <em>String</em></td>
<td>No</td>
<td>The ID of user who is responsible for accomplishing the current task. This makes sense for shared projects only. Accepts any user ID from the list of project collaborators or <code>null</code> or an empty string to unset.</td>
</tr>
<tr>
<td>auto_reminder <em>Boolean</em></td>
<td>No</td>
<td>When this option is enabled, the default reminder will be added to the new item if it has a due date with time set. See also the <a href="#tag/Sync/User">auto_reminder user option</a> for more info about the default reminder.</td>
</tr>
<tr>
<td>auto_parse_labels <em>Boolean</em></td>
<td>No</td>
<td>When this option is enabled, the labels will be parsed from the task content and added to the task. In case the label doesn&#39;t exist, a new one will be created.</td>
</tr>
<tr>
<td>duration <em>Object</em></td>
<td>No</td>
<td>The task&#39;s duration. Includes a positive integer (greater than zero) for the <code>amount</code> of time the task will take, and the <code>unit</code> of time that the amount represents which must be either <code>minute</code> or <code>day</code>. Both the <code>amount</code> and <code>unit</code> <strong>must</strong> be defined.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example update task request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "item_update",
        "uuid": "aca17834-da6f-4605-bde0-bd10be228878",
        "args": {
            "id": "6X7rM8997g3RQmvh",
            "content": "Buy Coffee",
            "due": {"string": "tomorrow at 10:00" }
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"318d16a7-0c88-46e0-9eb5-cde6c72477c8"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Updates task attributes. Please note that updating the parent, moving,
completing or uncompleting tasks is not supported by <code>item_update</code>, more
specific commands have to be used instead.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>The ID of the task.</td>
</tr>
<tr>
<td>content <em>String</em></td>
<td>No</td>
<td>The text of the task. This value may contain markdown-formatted text and hyperlinks. Details on markdown support can be found in the <a href="https://www.todoist.com/help/articles/format-text-in-a-todoist-task-e5dHw9">Text Formatting article</a> in the Help Center.</td>
</tr>
<tr>
<td>description <em>String</em></td>
<td>No</td>
<td>A description for the task. This value may contain markdown-formatted text and hyperlinks. Details on markdown support can be found in the <a href="https://www.todoist.com/help/articles/format-text-in-a-todoist-task-e5dHw9">Text Formatting article</a> in the Help Center.</td>
</tr>
<tr>
<td>due <em>Object</em></td>
<td>No</td>
<td>The due date of the task. See the <a href="#tag/Due-dates">Due dates</a> section for more details.</td>
</tr>
<tr>
<td>deadline <em>Object</em></td>
<td>No</td>
<td>The deadline of the task. See the <a href="#tag/Deadlines">Deadlines</a> section for more details.</td>
</tr>
<tr>
<td>priority <em>Integer</em></td>
<td>No</td>
<td>The priority of the task (a number between <code>1</code> and <code>4</code>, <code>4</code> for very urgent and <code>1</code> for natural). <br><strong>Note</strong>: Keep in mind that <code>very urgent</code> is the priority 1 on clients. So, <code>p1</code> will return <code>4</code> in the API.</td>
</tr>
<tr>
<td>is_collapsed <em>Boolean</em></td>
<td>No</td>
<td>Whether the task&#39;s sub-tasks are collapsed (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>labels <em>Array of String</em></td>
<td>No</td>
<td>The task&#39;s labels (a list of names that may represent either personal or shared labels).</td>
</tr>
<tr>
<td>assigned_by_uid <em>String</em></td>
<td>No</td>
<td>The ID of the user who assigned the task. This makes sense for shared projects only. Accepts <code>0</code> or any user ID from the list of project collaborators. If this value is unset or invalid, it will be automatically setup to your uid.</td>
</tr>
<tr>
<td>responsible_uid <em>String</em></td>
<td>No</td>
<td>The ID of the user who is responsible for accomplishing the task. This makes sense for shared projects only. Accepts any user ID from the list of project collaborators or <code>null</code> or an empty string to unset.</td>
</tr>
<tr>
<td>day_order <em>Integer</em></td>
<td>No</td>
<td>The order of the task inside the <code>Today</code> or <code>Next 7 days</code> view (a number, where the smallest value would place the task at the top).</td>
</tr>
<tr>
<td>duration <em>Object</em></td>
<td>No</td>
<td>The task&#39;s duration. Must a positive integer (greater than zero) for the <code>amount</code> of time the task will take, and the <code>unit</code> of time that the amount represents which must be either <code>minute</code> or <code>day</code>. Both the <code>amount</code> and <code>unit</code> <strong>must</strong> be defined. The object should be set to <code>null</code> to remove the task&#39;s duration.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example move task request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "item_move",
        "uuid": "318d16a7-0c88-46e0-9eb5-cde6c72477c8",
        "args": {
            "id": "6X7rM8997g3RQmvh",
            "parent_id": "6X7rf9x6pv2FGghW"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"318d16a7-0c88-46e0-9eb5-cde6c72477c8"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Move task to a different location. Only one of <code>parent_id</code>, <code>section_id</code> or
<code>project_id</code> must be set.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>The ID of the task.</td>
</tr>
<tr>
<td>parent_id <em>String</em></td>
<td>No</td>
<td>ID of the destination parent task. The task becomes the last child task of the parent task.</td>
</tr>
<tr>
<td>section_id <em>String</em></td>
<td>No</td>
<td>ID of the destination section. The task becomes the last root task of the section.</td>
</tr>
<tr>
<td>project_id <em>String</em></td>
<td>No</td>
<td>ID of the destination project. The task becomes the last root task of the project.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example reorder tasks request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token punctuation">[</span>
    <span class="token punctuation">{</span>
        <span class="token string">"type"</span><span class="token builtin class-name">:</span> <span class="token string">"item_reorder"</span>,
        <span class="token string">"uuid"</span><span class="token builtin class-name">:</span> <span class="token string">"bf0855a3-0138-4b76-b895-88cad8db9edc"</span>,
        <span class="token string">"args"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
            <span class="token string">"items"</span><span class="token builtin class-name">:</span> <span class="token punctuation">[</span>
                <span class="token punctuation">{</span><span class="token string">"id"</span><span class="token builtin class-name">:</span> <span class="token string">"6X7rM8997g3RQmvh"</span>, <span class="token string">"child_order"</span><span class="token builtin class-name">:</span> <span class="token number">1</span><span class="token punctuation">}</span>,
                <span class="token punctuation">{</span><span class="token string">"id"</span><span class="token builtin class-name">:</span> <span class="token string">"6X7rfFVPjhvv84XG"</span>, <span class="token string">"child_order"</span><span class="token builtin class-name">:</span> <span class="token number">2</span><span class="token punctuation">}</span>
            <span class="token punctuation">]</span>
        <span class="token punctuation">}</span>
    <span class="token punctuation">}</span><span class="token punctuation">]</span>'
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"bf0855a3-0138-4b76-b895-88cad8db9edc"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>The command updates <code>child_order</code> properties of items in bulk.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>items <em>Array of Objects</em></td>
<td>Yes</td>
<td>An array of objects to update. Each object contains two attributes: <code>id</code> of the item to update and <code>child_order</code>, the new order.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example delete task request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "item_delete",
        "uuid": "f8539c77-7fd7-4846-afad-3b201f0be8a5",
        "args": {"id": "6X7rfFVPjhvv84XG"}
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"f8539c77-7fd7-4846-afad-3b201f0be8a5"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Delete a task and all its sub-tasks.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>ID of the task to delete.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example complete task request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "item_complete",
        "uuid": "a74bfb5c-5f1d-4d14-baea-b7415446a871",
        "args": {
            "id": "6X7rfFVPjhvv84XG",
            "date_completed": "2017-01-02T01:00:00.000000Z"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"a74bfb5c-5f1d-4d14-baea-b7415446a871"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Completes a task and its sub-tasks and moves them to the archive. See also <code>item_close</code> for a
simplified version of the command.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>Task ID to complete.</td>
</tr>
<tr>
<td>date_completed <em>Date</em></td>
<td>No</td>
<td>RFC3339-formatted date of completion of the task (in UTC). If not set, the server will set the value to the current timestamp.</td>
</tr>
<tr>
<td>from_undo <em>Boolean</em></td>
<td>No</td>
<td>If <code>true</code>, skips incrementing completion stats. Used when restoring task state after undoing a completion.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example uncomplete task request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "item_uncomplete",
        "uuid": "710a60e1-174a-4313-bb9f-4df01e0349fd",
        "args": {"id": "2995104339"}
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"710a60e1-174a-4313-bb9f-4df01e0349fd"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>This command is used to uncomplete and restore an completed task.</p>
<p>Any ancestor items or sections will also be reinstated. Items will have the <code>checked</code> value reset.</p>
<p>The reinstated items and sections will appear at the end of the list within their parent, after any previously
active tasks.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>Task ID to uncomplete</td>
</tr>
</tbody></table>

<blockquote>
<p>Example complete recurring task request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span>'<span class="token punctuation">[</span>
    <span class="token punctuation">{</span>
        <span class="token string">"type"</span><span class="token builtin class-name">:</span> <span class="token string">"item_update_date_complete"</span>,
        <span class="token string">"uuid"</span><span class="token builtin class-name">:</span> <span class="token string">"c5888360-96b1-46be-aaac-b49b1135feab"</span>,
        <span class="token string">"args"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
            <span class="token string">"id"</span><span class="token builtin class-name">:</span> <span class="token string">"2995104339"</span>,
            <span class="token string">"due"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"date"</span><span class="token builtin class-name">:</span> <span class="token string">"2014-10-30"</span>, <span class="token string">"string"</span><span class="token builtin class-name">:</span> <span class="token string">"every day"</span><span class="token punctuation">}</span>,
            <span class="token string">"is_forward"</span><span class="token builtin class-name">:</span> <span class="token number">1</span>,
            <span class="token string">"reset_subtasks"</span><span class="token builtin class-name">:</span> <span class="token number">0</span>
        <span class="token punctuation">}</span>
    <span class="token punctuation">}</span><span class="token punctuation">]</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"c5888360-96b1-46be-aaac-b49b1135feab"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Complete a recurring task. The reason why this is a special case is because
we need to mark a recurring completion (and using <code>item_update</code> won&#39;t do
this). See also <code>item_close</code> for a simplified version of the command.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>The ID of the item to update (a number or a temp id).</td>
</tr>
<tr>
<td>due <em>Object</em></td>
<td>No</td>
<td>The due date of the task. See the <a href="#tag/Due-dates">Due dates</a> section for more details.</td>
</tr>
<tr>
<td>is_forward <em>Boolean</em></td>
<td>No</td>
<td>Set this argument to 1 for completion, or 0 for uncompletion (e.g., via undo). By default, this argument is set to 1 (completion).</td>
</tr>
<tr>
<td>reset_subtasks <em>Boolean</em></td>
<td>No</td>
<td>Set this property to 1 to reset subtasks when a recurring task is completed. By default, this property is not set (0), and subtasks will retain their existing status when the parent task recurs.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example close task request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "item_close",
        "uuid": "c5888360-96b1-46be-aaac-b49b1135feab",
        "args": {"id": "2995104339"}
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"c5888360-96b1-46be-aaac-b49b1135feab"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>A simplified version of <code>item_complete</code> / <code>item_update_date_complete</code>. The command
does exactly what official clients do when you close a task: regular tasks are
completed and moved to the archive, recurring tasks are scheduled to their next occurrence.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>The ID of the item to close (a number or a temp id).</td>
</tr>
</tbody></table>

<blockquote>
<p>Example update day orders request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "item_update_day_orders",
        "uuid": "dbeb40fc-905f-4d8a-8bae-547d3bbd6e91",
        "args": {"ids_to_orders": {"2995104339": 1}}
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"dbeb40fc-905f-4d8a-8bae-547d3bbd6e91"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Update the day orders of multiple tasks at once.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>ids_to_orders <em>Object</em></td>
<td>Yes</td>
<td>A dictionary, where a task <code>id</code> is the key, and the <code>day_order</code> value: <code>item_id: day_order</code>.</td>
</tr>
</tbody></table>

<p><em>Availability of filters functionality and the maximum number of saved filters are dependent
on the current user plan. These values are indicated by the <code>filters</code> and <code>max_filters</code>
properties of the <a href="#tag/Sync/User/User-plan-limits">user plan limits</a> object.</em></p>
<blockquote>
<p>An example filter:</p>
</blockquote>
<pre><code><span class="token punctuation">{</span>
    <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"4638878"</span><span class="token punctuation">,</span>
    <span class="token string">"name"</span><span class="token punctuation">:</span> <span class="token string">"Important"</span><span class="token punctuation">,</span>
    <span class="token string">"query"</span><span class="token punctuation">:</span> <span class="token string">"priority 1"</span><span class="token punctuation">,</span>
    <span class="token string">"color"</span><span class="token punctuation">:</span> <span class="token string">"lime_green"</span><span class="token punctuation">,</span>
    <span class="token string">"item_order"</span><span class="token punctuation">:</span> <span class="token number">3</span><span class="token punctuation">,</span>
    <span class="token string">"is_deleted"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"is_favorite"</span><span class="token punctuation">:</span> <span class="token boolean">false</span>
    <span class="token string">"is_frozen"</span><span class="token punctuation">:</span> <span class="token boolean">false</span>
<span class="token punctuation">}</span>
</code></pre>
<h4 id="properties">Properties</h4>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>The ID of the filter.</td>
</tr>
<tr>
<td>name <em>String</em></td>
<td>The name of the filter.</td>
</tr>
<tr>
<td>query <em>String</em></td>
<td>The query to search for. <a href="https://www.todoist.com/help/articles/introduction-to-filters-V98wIH">Examples of searches</a> can be found in the Todoist help page.</td>
</tr>
<tr>
<td>color <em>String</em></td>
<td>The color of the filter icon. Refer to the <code>name</code> column in the <a href="#tag/Colors">Colors</a> guide for more info.</td>
</tr>
<tr>
<td>item_order <em>Integer</em></td>
<td>Filter’s order in the filter list (where the smallest value should place the filter at the top).</td>
</tr>
<tr>
<td>is_deleted <em>Boolean</em></td>
<td>Whether the filter is marked as deleted (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>is_favorite <em>Boolean</em></td>
<td>Whether the filter is a favorite (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>is_frozen <em>Boolean</em></td>
<td>Filters from a cancelled subscription cannot be changed. This is a read-only attribute (a <code>true</code> or <code>false</code> value).</td>
</tr>
</tbody></table>

<blockquote>
<p>Example add filter request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "filter_add",
        "temp_id": "9204ca9f-e91c-436b-b408-ea02b3972686",
        "uuid": "0b8690b8-59e6-4d5b-9c08-6b4f1e8e0eb8",
        "args": {
            "name": "Important",
            "query": "priority 1"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"0b8690b8-59e6-4d5b-9c08-6b4f1e8e0eb8"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token string">"temp_id_mapping"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"9204ca9f-e91c-436b-b408-ea02b3972686"</span><span class="token builtin class-name">:</span> <span class="token string">"4638878"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>name <em>String</em></td>
<td>Yes</td>
<td>The name of the filter.</td>
</tr>
<tr>
<td>query <em>String</em></td>
<td>Yes</td>
<td>The query to search for. <a href="https://www.todoist.com/help/articles/introduction-to-filters-V98wIH">Examples of searches</a> can be found in the Todoist help page.</td>
</tr>
<tr>
<td>color <em>String</em></td>
<td>No</td>
<td>The color of the filter icon. Refer to the <code>name</code> column in the <a href="#tag/Colors">Colors</a> guide for more info.</td>
</tr>
<tr>
<td>item_order <em>Integer</em></td>
<td>No</td>
<td>Filter’s order in the filter list (the smallest value should place the filter at the top).</td>
</tr>
<tr>
<td>is_favorite <em>Boolean</em></td>
<td>No</td>
<td>Whether the filter is a favorite (a <code>true</code> or <code>false</code> value).</td>
</tr>
</tbody></table>

<blockquote>
<p>Example update filter request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "filter_update",
        "uuid": "a68b588a-44f7-434c-b3c5-a699949f755c",
        "args": {
            "id": "4638879",
            "name": "Not Important"
            "query": "priority 4"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"a68b588a-44f7-434c-b3c5-a699949f755c"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>The ID of the filter.</td>
</tr>
<tr>
<td>name <em>String</em></td>
<td>No</td>
<td>The name of the filter</td>
</tr>
<tr>
<td>query <em>String</em></td>
<td>No</td>
<td>The query to search for. <a href="https://www.todoist.com/help/articles/introduction-to-filters-V98wIH">Examples of searches</a> can be found in the Todoist help page.</td>
</tr>
<tr>
<td>color <em>String</em></td>
<td>No</td>
<td>The color of the filter icon. Refer to the <code>name</code> column in the <a href="#tag/Colors">Colors</a> guide for more info.</td>
</tr>
<tr>
<td>item_order <em>Integer</em></td>
<td>No</td>
<td>Filter’s order in the filter list (where the smallest value should place the filter at the top).</td>
</tr>
<tr>
<td>is_favorite <em>Boolean</em></td>
<td>No</td>
<td>Whether the filter is a favorite (a <code>true</code> or <code>false</code> value).</td>
</tr>
</tbody></table>

<blockquote>
<p>Example delete filter request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[{"type": "filter_delete", "uuid": "b8186025-66d5-4eae-b0dd-befa541abbed", "args": {"id": "9"}}]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"b8186025-66d5-4eae-b0dd-befa541abbed"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>The ID of the filter.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example reorder filters request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token punctuation">[</span>
    <span class="token punctuation">{</span>
        <span class="token string">"type"</span><span class="token builtin class-name">:</span> <span class="token string">"filter_update_orders"</span>,
        <span class="token string">"uuid"</span><span class="token builtin class-name">:</span> <span class="token string">"517560cc-f165-4ff6-947b-3adda8aef744"</span>,
        <span class="token string">"args"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
            <span class="token string">"id_order_mapping"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"4638878"</span><span class="token builtin class-name">:</span>  <span class="token number">1</span>, <span class="token string">"4638879"</span><span class="token builtin class-name">:</span> <span class="token number">2</span><span class="token punctuation">}</span>
        <span class="token punctuation">}</span>
    <span class="token punctuation">}</span><span class="token punctuation">]</span>'
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"517560cc-f165-4ff6-947b-3adda8aef744"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Update the orders of multiple filters at once.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id_order_mapping <em>Object</em></td>
<td>Yes</td>
<td>A dictionary, where a filter ID is the key, and the order its value: <code>filter_id: order</code>.</td>
</tr>
</tbody></table>

<p><em>The maximum number of saved filters are dependent on the workspaces current plan.
These values are indicated by the <code>max_filters</code> property inside <code>limits</code> on the
workspace object</em></p>
<blockquote>
<p>An example workspace filter:</p>
</blockquote>
<pre><code><span class="token punctuation">{</span>
    <span class="token string">"id"</span><span class="token punctuation">:</span> <span class="token string">"123456"</span><span class="token punctuation">,</span>
    <span class="token string">"workspace_id"</span><span class="token punctuation">:</span> <span class="token string">"789012"</span><span class="token punctuation">,</span>
    <span class="token string">"name"</span><span class="token punctuation">:</span> <span class="token string">"Team Priorities"</span><span class="token punctuation">,</span>
    <span class="token string">"query"</span><span class="token punctuation">:</span> <span class="token string">"priority 1 &amp; assigned to: team"</span><span class="token punctuation">,</span>
    <span class="token string">"color"</span><span class="token punctuation">:</span> <span class="token string">"red"</span><span class="token punctuation">,</span>
    <span class="token string">"item_order"</span><span class="token punctuation">:</span> <span class="token number">1</span><span class="token punctuation">,</span>
    <span class="token string">"is_deleted"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"is_favorite"</span><span class="token punctuation">:</span> <span class="token boolean">true</span><span class="token punctuation">,</span>
    <span class="token string">"is_frozen"</span><span class="token punctuation">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string">"creator_uid"</span><span class="token punctuation">:</span> <span class="token string">"111222"</span><span class="token punctuation">,</span>
    <span class="token string">"updater_uid"</span><span class="token punctuation">:</span> <span class="token string">"111222"</span><span class="token punctuation">,</span>
    <span class="token string">"created_at"</span><span class="token punctuation">:</span> <span class="token string">"2024-01-15T10:00:00Z"</span><span class="token punctuation">,</span>
    <span class="token string">"updated_at"</span><span class="token punctuation">:</span> <span class="token string">"2024-01-15T11:00:00Z"</span>
<span class="token punctuation">}</span>
</code></pre>
<h4 id="properties">Properties</h4>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>The ID of the workspace filter.</td>
</tr>
<tr>
<td>workspace_id <em>String</em></td>
<td>The ID of the workspace this filter belongs to.</td>
</tr>
<tr>
<td>name <em>String</em></td>
<td>The name of the workspace filter.</td>
</tr>
<tr>
<td>query <em>String</em></td>
<td>The query to search for. <a href="https://www.todoist.com/help/articles/introduction-to-filters-V98wIH">Examples of searches</a> can be found in the Todoist help page.</td>
</tr>
<tr>
<td>color <em>String</em></td>
<td>The color of the filter icon. Refer to the <code>name</code> column in the <a href="#tag/Colors">Colors</a> guide for more info.</td>
</tr>
<tr>
<td>item_order <em>Integer</em></td>
<td>Filter&#39;s order in the filter list (where the smallest value should place the filter at the top).</td>
</tr>
<tr>
<td>is_deleted <em>Boolean</em></td>
<td>Whether the filter is marked as deleted (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>is_favorite <em>Boolean</em></td>
<td>Whether the filter is a favorite for the user (note: not at workspace level) (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>is_frozen <em>Boolean</em></td>
<td>Filters created outside plan limits (through cancellation, downgrade, etc) cannot be changed. This is a read-only attribute (a <code>true</code> or <code>false</code> value).</td>
</tr>
<tr>
<td>creator_uid <em>String</em></td>
<td>The ID of the user who created the workspace filter.</td>
</tr>
<tr>
<td>updater_uid <em>String</em></td>
<td>The ID of the user who last updated the workspace filter.</td>
</tr>
<tr>
<td>created_at <em>String</em></td>
<td>The date when the workspace filter was created (RFC3339 format in UTC).</td>
</tr>
<tr>
<td>updated_at <em>String</em></td>
<td>The date when the workspace filter was last updated (RFC3339 format in UTC).</td>
</tr>
</tbody></table>

<blockquote>
<p>Example add workspace filter request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "workspace_filter_add",
        "temp_id": "9204ca9f-e91c-436b-b408-ea02b3972686",
        "uuid": "0b8690b8-59e6-4d5b-9c08-6b4f1e8e0eb8",
        "args": {
            "workspace_id": "789012",
            "name": "Team Priorities",
            "query": "priority 1 &amp; assigned to: team"
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"0b8690b8-59e6-4d5b-9c08-6b4f1e8e0eb8"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token string">"temp_id_mapping"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"9204ca9f-e91c-436b-b408-ea02b3972686"</span><span class="token builtin class-name">:</span> <span class="token string">"123456"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>workspace_id <em>String or Integer</em></td>
<td>Yes</td>
<td>The ID of the workspace this filter belongs to.</td>
</tr>
<tr>
<td>name <em>String</em></td>
<td>Yes</td>
<td>The name of the workspace filter.</td>
</tr>
<tr>
<td>query <em>String</em></td>
<td>Yes</td>
<td>The query to search for. <a href="https://www.todoist.com/help/articles/introduction-to-filters-V98wIH">Examples of searches</a> can be found in the Todoist help page.</td>
</tr>
<tr>
<td>color <em>String</em></td>
<td>No</td>
<td>The color of the filter icon. Refer to the <code>name</code> column in the <a href="#tag/Colors">Colors</a> guide for more info.</td>
</tr>
<tr>
<td>item_order <em>Integer</em></td>
<td>No</td>
<td>Filter&#39;s order in the filter list (the smallest value should place the filter at the top).</td>
</tr>
</tbody></table>

<blockquote>
<p>Example update workspace filter request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[
    {
        "type": "workspace_filter_update",
        "uuid": "a68b588a-44f7-434c-b3c5-a699949f755c",
        "args": {
            "id": "123456",
            "name": "High Priority Team Tasks",
            "query": "priority 1 &amp; assigned to: team",
            "is_favorite": true
        }
    }]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"a68b588a-44f7-434c-b3c5-a699949f755c"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>The ID of the workspace filter.</td>
</tr>
<tr>
<td>name <em>String</em></td>
<td>No</td>
<td>The name of the workspace filter.</td>
</tr>
<tr>
<td>query <em>String</em></td>
<td>No</td>
<td>The query to search for. <a href="https://www.todoist.com/help/articles/introduction-to-filters-V98wIH">Examples of searches</a> can be found in the Todoist help page.</td>
</tr>
<tr>
<td>color <em>String</em></td>
<td>No</td>
<td>The color of the filter icon. Refer to the <code>name</code> column in the <a href="#tag/Colors">Colors</a> guide for more info.</td>
</tr>
<tr>
<td>item_order <em>Integer</em></td>
<td>No</td>
<td>Filter&#39;s order in the filter list (where the smallest value should place the filter at the top).</td>
</tr>
<tr>
<td>is_favorite <em>Boolean</em></td>
<td>No</td>
<td>Whether the filter is a favorite for the user (a <code>true</code> or <code>false</code> value).</td>
</tr>
</tbody></table>

<blockquote>
<p>Example delete workspace filter request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token string">'[{"type": "workspace_filter_delete", "uuid": "b8186025-66d5-4eae-b0dd-befa541abbed", "args": {"id": "123456"}}]'</span>
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"b8186025-66d5-4eae-b0dd-befa541abbed"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>String</em></td>
<td>Yes</td>
<td>The ID of the workspace filter.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example reorder workspace filters request:</p>
</blockquote>
<pre><code class="language-shell">$ <span class="token function">curl</span> https://api.todoist.com/api/v1/sync <span class="token punctuation">\</span>
    <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer 0123456789abcdef0123456789abcdef01234567"</span> <span class="token punctuation">\</span>
    <span class="token parameter variable">-d</span> <span class="token assign-left variable">commands</span><span class="token operator">=</span><span class="token punctuation">[</span>
    <span class="token punctuation">{</span>
        <span class="token string">"type"</span><span class="token builtin class-name">:</span> <span class="token string">"workspace_filter_update_orders"</span>,
        <span class="token string">"uuid"</span><span class="token builtin class-name">:</span> <span class="token string">"517560cc-f165-4ff6-947b-3adda8aef744"</span>,
        <span class="token string">"args"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span>
            <span class="token string">"id_order_mapping"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"123456"</span><span class="token builtin class-name">:</span>  <span class="token number">1</span>, <span class="token string">"123457"</span><span class="token builtin class-name">:</span> <span class="token number">2</span><span class="token punctuation">}</span>
        <span class="token punctuation">}</span>
    <span class="token punctuation">}</span><span class="token punctuation">]</span>'
</code></pre>
<blockquote>
<p>Example response:</p>
</blockquote>
<pre><code class="language-shell"><span class="token punctuation">{</span>
  <span class="token punctuation">..</span>.
  <span class="token string">"sync_status"</span><span class="token builtin class-name">:</span> <span class="token punctuation">{</span><span class="token string">"517560cc-f165-4ff6-947b-3adda8aef744"</span><span class="token builtin class-name">:</span> <span class="token string">"ok"</span><span class="token punctuation">}</span>,
  <span class="token punctuation">..</span>.
<span class="token punctuation">}</span>
</code></pre>
<p>Update the orders of multiple workspace filters at once.</p>
<h4 id="command-arguments">Command arguments</h4>
<table>
<thead>
<tr>
<th>Argument</th>
<th>Required</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id_order_mapping <em>Object</em></td>
<td>Yes</td>
<td>A dictionary, where a workspace filter ID is the key, and the order its value: <code>filter_id: order</code>.</td>
</tr>
</tbody></table>
<p><strong>Key differences from personal filters:</strong></p>
<ul>
<li>Workspace filters require membership in the associated workspace</li>
<li>Changes propagate to all workspace members via sync events</li>
<li>Permissions are checked through workspace membership rather than user ownership</li>
</ul>

<p>Endpoints related to ID mappings between v1 and v2</p>

<p>Translates IDs from v1 to v2 or vice versa.</p>
<p>IDs are not unique across object types, hence the need to specify the object type.</p>
<p>When V1 ids are provided, the function will return the corresponding V2
ids, if they exist, and vice versa.</p>
<p>When no objects are found, an empty list is returned.</p>

<p>A comma-separated list of IDs</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Deletes a workspace invitation. Only admins can delete invitations.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Return a list containing details of all pending invitation to a workspace.</p>
<p>This list is not paginated. All workspace members can access this list.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Accept a workspace invitation. Usable by authenticated users only.</p>

<p>An opaque string representing an invite code. This invitation code is sent to a user via email and is exclusive for the user.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Reject a workspace invitation. Usable by authenticated users only.</p>

<p>An opaque string representing an invite code. This invitation code is sent to a user via email and is exclusive for the user.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Returns all active workspace projects, including those visible but not joined by the user.</p>
<p><em>For guests, returns all joined workspace projects only.</em></p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Lists details of the workspace&#39;s current plan and usage</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Return a list of user emails who have a pending invitation to a workspace.</p>
<p>The list is not paginated. All workspace members can access this list.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Returns all workspace_users for a given workspace if workspace_id is
provided. Otherwise, returns all workspace_users for all workspaces that
the requesting user is part of.</p>
<p><em>Not accessible by guests.</em></p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Join a workspace via link or via workspace ID, if the user can auto-join
the workspace by domain.</p>
<h2 id="joining-by-domain">Joining by Domain</h2>
<p>This is possible if:</p>
<ul>
<li>The user is verified</li>
<li>The user has a user e-mail belonging to a domain that is set
as a domain name for a workspace</li>
<li>That workspace has the auto-join by domain feature enabled</li>
</ul>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Upload an image to be used as the workspace logo. Similar to a user’s
avatar. If <code>delete</code> is set to true, it removes the logo completely and does
not return any <code>logo_*</code> attribute.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>The number of objects to return in a page</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Search active user projects by name.</p>
<p>This is a paginated endpoint. See the <a href="#tag/Pagination">Pagination guide</a> for details on using cursor-based pagination.</p>

<p>Search query to match project names. Matching is case-insensitive. Queries are matched literally unless <code>*</code> (wildcard) is included. Use <code>\*</code> for literal asterisk and <code>\\</code> for literal backslash.</p>

<p>The number of objects to return in a page</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Creates a new project and returns it</p>

<p>Name of the project</p>

<p>Description of the project</p>

<p>Parent project ID. If provided, creates this project as a sub-project</p>

<p>Color of the project icon</p>

<p>Whether the project is a favorite for the user</p>

<p>View style of the project</p>

<p>Workspace ID. If provided, creates a workspace project instead of a personal project</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Get all active user projects.</p>
<p>This is a paginated endpoint. See the <a href="#tag/Pagination">Pagination guide</a> for details on using cursor-based pagination.</p>

<p>The number of objects to return in a page</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Get all collaborators for a given project.</p>
<p>This is a paginated endpoint. See the <a href="#tag/Pagination">Pagination guide</a> for details on using cursor-based pagination.</p>

<p>String ID of the project</p>

<p>The number of objects to return in a page</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Marks a previously archived project as active again. For personal projects, this
will make the project visible again for the initiating user. For workspace projects,
this will make the project visible again for all applicable workspace users.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Marks a project as archived. For personal projects, this will archive it just for
the initiating user (leaving it visible to any other collaborators). For workspace
projects, this will archive it for all workspace users, removing it from view.</p>

<p>String ID of the project</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Returns a list of all the available roles and the associated actions they can
perform in a project.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p><em>Only used for workspaces</em></p>
<p>This endpoint is used to join a workspace project by a workspace_user and
is only usable by the workspace user.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Returns a project object related to the given ID</p>

<p>String ID of the project</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Updated a project and return it</p>

<p>String ID of the project</p>

<p>Updated project name. Passing null or omitting this field will leave it unchanged.</p>

<p>Updated project description. Passing null or omitting this field will leave it unchanged.</p>

<p>Updated project color. Passing null or omitting this field will leave it unchanged.</p>

<p>Whether the project is marked as a favorite. Passing null or omitting this field will leave it unchanged.</p>

<p>Updated project view style. Passing null or omitting this field will leave it unchanged.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Deletes a project and all of its sections and tasks.</p>

<p>String ID of the project</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Some objects (like projects, labels, and filters) returned by our APIs may have
colors defined by an id or name. The table below shows all information you may
need for any of these colors.</p>
<table>
<thead>
<tr>
<th>ID</th>
<th>Name</th>
<th>Hexadecimal</th>
<th></th>
<th>ID</th>
<th>Name</th>
<th>Hexadecimal</th>
</tr>
</thead>
<tbody><tr>
<td>30</td>
<td>berry_red</td>
<td><span class="color-block" style="background-color: #B8255F"></span> <code>#B8255F</code></td>
<td></td>
<td>40</td>
<td>light_blue</td>
<td><span class="color-block" style="background-color: #6988A4"></span> <code>#6988A4</code></td>
</tr>
<tr>
<td>31</td>
<td>red</td>
<td><span class="color-block" style="background-color: #DC4C3E"></span> <code>#DC4C3E</code></td>
<td></td>
<td>41</td>
<td>blue</td>
<td><span class="color-block" style="background-color: #4180FF"></span> <code>#4180FF</code></td>
</tr>
<tr>
<td>32</td>
<td>orange</td>
<td><span class="color-block" style="background-color: #C77100"></span> <code>#C77100</code></td>
<td></td>
<td>42</td>
<td>grape</td>
<td><span class="color-block" style="background-color: #692EC2"></span> <code>#692EC2</code></td>
</tr>
<tr>
<td>33</td>
<td>yellow</td>
<td><span class="color-block" style="background-color: #B29104"></span> <code>#B29104</code></td>
<td></td>
<td>43</td>
<td>violet</td>
<td><span class="color-block" style="background-color: #CA3FEE"></span> <code>#CA3FEE</code></td>
</tr>
<tr>
<td>34</td>
<td>olive_green</td>
<td><span class="color-block" style="background-color: #949C31"></span> <code>#949C31</code></td>
<td></td>
<td>44</td>
<td>lavender</td>
<td><span class="color-block" style="background-color: #A4698C"></span> <code>#A4698C</code></td>
</tr>
<tr>
<td>35</td>
<td>lime_green</td>
<td><span class="color-block" style="background-color: #65A33A"></span> <code>#65A33A</code></td>
<td></td>
<td>45</td>
<td>magenta</td>
<td><span class="color-block" style="background-color: #E05095"></span> <code>#E05095</code></td>
</tr>
<tr>
<td>36</td>
<td>green</td>
<td><span class="color-block" style="background-color: #369307"></span> <code>#369307</code></td>
<td></td>
<td>46</td>
<td>salmon</td>
<td><span class="color-block" style="background-color: #C9766F"></span> <code>#C9766F</code></td>
</tr>
<tr>
<td>37</td>
<td>mint_green</td>
<td><span class="color-block" style="background-color: #42A393"></span> <code>#42A393</code></td>
<td></td>
<td>47</td>
<td>charcoal</td>
<td><span class="color-block" style="background-color: #808080"></span> <code>#808080</code></td>
</tr>
<tr>
<td>38</td>
<td>teal</td>
<td><span class="color-block" style="background-color: #148FAD"></span> <code>#148FAD</code></td>
<td></td>
<td>48</td>
<td>grey</td>
<td><span class="color-block" style="background-color: #999999"></span> <code>#999999</code></td>
</tr>
<tr>
<td>39</td>
<td>sky_blue</td>
<td><span class="color-block" style="background-color: #319DC0"></span> <code>#319DC0</code></td>
<td></td>
<td>49</td>
<td>taupe</td>
<td><span class="color-block" style="background-color: #8F7A69"></span> <code>#8F7A69</code></td>
</tr>
</tbody></table>

<p>Creates a new comment on a project or task and returns it.</p>
<p>Exactly one of <code>task_id</code> or <code>project_id</code> arguments is required. Providing
neither or both will return an error.</p>

<p>Content of the comment</p>

<p>String ID of the project</p>

<p>String ID of the task</p>

<p>A <a href="#tag/Sync/Comments/File-Attachments">File attachment</a> object</p>

<p>Optional list of user IDs to notify about this comment.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Get all comments for a given task or project.</p>
<p>Exactly one of <code>task_id</code> or <code>project_id</code> arguments is required. Providing
neither or both will return an error.</p>
<p>This is a paginated endpoint. See the <a href="#tag/Pagination">Pagination guide</a> for details on using cursor-based pagination.</p>

<p>String ID of the project</p>

<p>String ID of the task</p>

<p>The number of objects to return in a page</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Returns a single comment by ID</p>

<p>String ID of the comment</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Update a comment by ID and returns its content</p>

<p>String ID of the comment</p>

<p>New content for the comment. If null or an empty string, no update is performed.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Delete a comment by ID</p>

<p>String ID of the comment</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Templates allow exporting of a project&#39;s tasks to a file or URL, and then
importing of the task list to a new or existing project.</p>
<p>Availability of project templates functionality is dependent on the current
user plan. This values is indicated by the <code>templates</code> property of the <a href="#tag/Sync/User/User-plan-limits">user
plan limits</a> object.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>A template can be imported in an existing project, or in a newly created one.</p>
<p>Upload a file suitable to be passed as a template to be imported into a project.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>A template can be imported in an existing project, or in a newly created one.</p>
<p>Upload a file suitable to be passed as a template to be imported into a project.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Get a template for a project as a CSV file</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Get a template for a project as a shareable URL.</p>
<p>The URL can then be passed to <code>https://todoist.com/api/v1/import/project_from_url?t_url=&lt;url&gt;</code>
to make a shareable template.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Search active sections by name, optionally filtered by project.</p>
<p>This is a paginated endpoint. See the <a href="#tag/Pagination">Pagination guide</a> for details on using cursor-based pagination.</p>

<p>Search query to match section names. Matching is case-insensitive. Queries are matched literally unless <code>*</code> (wildcard) is included. Use <code>\*</code> for literal asterisk and <code>\\</code> for literal backslash.</p>

<p>String ID of the project to search sections from. If omitted or null, search sections from all projects.</p>

<p>The number of objects to return in a page</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Create a new section</p>

<p>Name of the new section</p>

<p>ID of the project to add the section to</p>

<p>Position of the new section in the project</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Get all active sections for the user, optionally filtered by project.</p>
<p>This is a paginated endpoint. See the <a href="#tag/Pagination">Pagination guide</a> for details on using cursor-based pagination.</p>

<p>String ID of the project to get sections from. If omitted or null, get sections from all projects.</p>

<p>The number of objects to return in a page</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Return the section for the given section ID</p>

<p>String ID of the section</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>String ID of the section</p>

<p>Updated section name. Passing null or omitting this field will leave it unchanged.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Delete the section and all of its tasks</p>

<p>String ID of the section</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Create a new task.</p>

<p>Task content</p>

<p>Task description</p>

<p>ID of the project to add the task to. If omitted or null, the task will be added to the user&#39;s Inbox.</p>

<p>ID of the section to add the task to</p>

<p>ID of the parent task</p>

<p>Position of the task in the project or section</p>

<p>List of label names</p>

<p>Task priority (1-4, where 1 is highest)</p>

<p>ID of the user to assign the task to</p>

<p>Human-readable representation of the due date. See the <a href="#tag/Due-dates">Due dates</a> section for more details.</p>

<p>Due date in RFC 3339 format or similar. See the <a href="#tag/Due-dates">Due dates</a> section for more details.</p>

<p>Due date and time. See the <a href="#tag/Due-dates">Due dates</a> section for more details.</p>

<p>Due date language code. See the <a href="#tag/Due-dates">Due dates</a> section for more details.</p>

<p>Task duration, in either minutes or days. Only used if <code>duration_unit</code> is also provided.</p>

<p>Unit of time for duration</p>

<p>Deadline date in YYYY-MM-DD format</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Get all active tasks for the user.</p>
<p>All provided parameters are used to narrow down the list of tasks.</p>
<p>This is a paginated endpoint. See the <a href="#tag/Pagination">Pagination guide</a> for details on using cursor-based pagination.</p>

<p>String ID of the project to get tasks from</p>

<p>String ID of the section to get tasks from</p>

<p>String ID of the parent task to get sub-tasks from</p>

<p>Filter tasks by label name</p>

<p>A list of the task IDs to retrieve, this should be a comma separated list</p>

<p>The number of objects to return in a page</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Retrieves a list of completed tasks strictly limited by the specified completion
date range (up to 3 months).</p>
<p>It can retrieve completed items:</p>
<ul>
<li>From all the projects the user has joined in a workspace</li>
<li>From all the projects of the user</li>
<li>That match many <a href="https://todoist.com/help/articles/introduction-to-filters-V98wIH">supported
filters</a></li>
</ul>
<p>By default, the response is limited to a page containing a maximum of 50 items
(configurable using <code>limit</code>).</p>
<p>Subsequent pages of results can be fetched by using the <code>next_cursor</code> value from the
response as the <code>cursor</code> value for the next request.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Retrieves a list of completed items strictly limited by the specified due date range
(up to 6 weeks).</p>
<p>It can retrieve completed items:</p>
<ul>
<li>From within a project, section, or parent item</li>
<li>From all the projects the user has joined in a workspace</li>
<li>From all the projects of the user</li>
<li>That match many <a href="https://todoist.com/help/articles/introduction-to-filters-V98wIH">supported
filters</a></li>
</ul>
<p>By default, the response is limited to a page containing a maximum of 50 items
(configurable using <code>limit</code>).</p>
<p>Subsequent pages of results can be fetched by using the <code>next_cursor</code> value from the
response as the <code>cursor</code> value for the next request.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Get all tasks matching the filter.</p>
<p>This is a paginated endpoint. See the <a href="#tag/Pagination">Pagination guide</a> for details on using cursor-based pagination.</p>

<p>Filter by any <a href="https://todoist.com/help/articles/introduction-to-filters-V98wIH">supported filter</a>. Multiple filters (using the comma <code>,</code> operator) are not supported.</p>

<p>IETF language tag defining what language filter is written in, if differs from default English</p>

<p>The number of objects to return in a page</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Add a new task using the Quick Add implementation similar to that used in
the official clients</p>

<p>The text of the task that is parsed. It can include a due date in free form text, a project name starting with the <code>#</code> character (without spaces), a label starting with the <code>@</code> character, an assignee starting with the <code>+</code> character, a priority (e.g., <code>p1</code>), a deadline between <code>{}</code> (e.g. {in 3 days}), or a description starting from <code>//</code> until the end of the text.</p>

<p>The reminder date in free form text.</p>

<p>When this option is enabled, the default reminder will be added to the new item if it has a due date with time set. See also the <a href="#tag/Sync/User">auto_reminder user option</a> for more info about the default reminder.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Reopens a task.</p>
<p>Any ancestor tasks or sections will also be marked as uncomplete and
restored from history.</p>
<p>The reinstated tasks and sections will appear at the end of the list within
their parent, after any previously active tasks.</p>

<p>String ID of the task</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Closes a task.</p>
<p>The command performs in the same way as our official clients:</p>
<p>Regular tasks are marked complete and moved to history, along with their
subtasks. Tasks with <a href="https://todoist.com/help/articles/introduction-to-recurring-dates-YUYVJJAV">recurring due
dates</a>
will be scheduled to their next occurrence.</p>

<p>String ID of the task</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Moves task to another project, section or parent.</p>

<p>ID of the task to move</p>

<p>ID of the project to move the task to</p>

<p>ID of the section to move the task to</p>

<p>ID of the parent task to move the task under</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Returns a single active (non-completed) task by ID</p>

<p>String ID of the task</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Updates an existing task.</p>

<p>String ID of the task</p>

<p>Updated task content. Omit this field to keep it unchanged.</p>

<p>Updated task description. Omit this field to keep it unchanged.</p>

<p>Updated list of label names. Omit this field to keep it unchanged.</p>

<p>Updated task priority (1-4, where 1 is highest). Omit this field to keep it unchanged.</p>

<p>Updated human-readable representation of the due date. See the <a href="#tag/Due-dates">Due dates</a> section for more details. Omit this field to keep it unchanged.</p>

<p>Updated due date in RFC 3339 format or similar. See the <a href="#tag/Due-dates">Due dates</a> section for more details. Omit this field to keep it unchanged.</p>

<p>Updated due date and time. See the <a href="#tag/Due-dates">Due dates</a> section for more details. Omit this field to keep it unchanged.</p>

<p>Updated due date language code. See the <a href="#tag/Due-dates">Due dates</a> section for more details. Omit this field to keep it unchanged.</p>

<p>ID of the user to assign the task to. Pass null to clear the value. Omit this field to keep it unchanged.</p>

<p>Updated task duration, in either minutes or days. Only used if <code>duration_unit</code> is also provided. Pass null to clear the value. Omit this field to keep it unchanged.</p>

<p>Unit of time for duration. Must be provided to update the task duration. Pass null to clear the value. Omit this field to keep it unchanged.</p>

<p>Updated deadline date in YYYY-MM-DD format. Pass null to clear the value. Omit this field to keep it unchanged.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>String ID of the task</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Search user labels by name.</p>
<p>This is a paginated endpoint. See the <a href="#tag/Pagination">Pagination guide</a> for details on using cursor-based pagination.</p>

<p>Search query to match label names. Matching is case-insensitive. Queries are matched literally unless <code>*</code> (wildcard) is included. Use <code>\*</code> for literal asterisk and <code>\\</code> for literal backslash.</p>

<p>The number of objects to return in a page</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Returns a set of unique strings containing labels from active tasks.</p>
<p>By default, the names of a user&#39;s personal labels will also be included.
These can be excluded by passing the <code>omit_personal</code> parameter.</p>

<p>The number of objects to return in a page</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Get all user labels.</p>
<p>This is a paginated endpoint. See the <a href="#tag/Pagination">Pagination guide</a> for details on using cursor-based pagination.</p>

<p>The number of objects to return in a page</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Name of the new label</p>

<p>Position of the new label in the label list</p>

<p>Label color</p>

<p>Whether the label is marked as a favorite</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Remove the given shared label from all active tasks</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Rename the given shared label from all active tasks</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Deletes a personal label. All instances of the label will be removed from tasks</p>

<p>String ID of the label</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>String ID of the label</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>String ID of the label</p>

<p>Updated label name. Passing null or omitting this field will leave it unchanged.</p>

<p>Position of the label in the label list. Passing null or omitting this field will leave it unchanged.</p>

<p>Label color. Passing null or omitting this field will leave it unchanged.</p>

<p>Whether the label is marked as a favorite. Passing null or omitting this field will leave it unchanged.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Availability of uploads functionality and the maximum size for a file
attachment are dependent on the current user plan. These values are indicated
by the <code>uploads</code> and <code>upload_limit_mb</code> properties of the user plan limits object.</p>
<p>Files can be uploaded to our servers and used as <a href="#tag/Sync/Comments/File-Attachments">File
Attachments</a> in <a href="#tag/Comments">comments</a>.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Upload a file to Todoist.</p>
<p>This endpoint accepts file uploads via two methods:</p>
<ol>
<li><p><strong>Multipart form-data</strong> (recommended):</p>
<ul>
<li>Send the file as a form field with the actual file content</li>
<li>Optionally include <code>project_id</code> as another form field</li>
<li>The filename will be extracted from the Content-Disposition header</li>
</ul>
</li>
<li><p><strong>Raw binary stream</strong>:</p>
<ul>
<li>Send the file content directly in the request body</li>
<li>Set <code>Content-Type</code> header to the file&#39;s MIME type</li>
<li>Set <code>X-File-Name</code> header with the desired filename</li>
<li>Optionally include <code>project_id</code> as a query parameter</li>
</ul>
</li>
</ol>
<p>The optional <code>project_id</code> parameter can be used to apply workspace-specific
upload limits when uploading to a workspace project.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Everything managed via <code>/sync</code> endpoint</p>

<p>Everything managed via <code>/sync</code> endpoint</p>

<p>Due dates for tasks and reminders is one of the core concepts of Todoist. It&#39;s
very powerful and quite complex, because it has to embrace different use-cases
of Todoist users.</p>
<p>Todoist supports three types of due dates.</p>
<ul>
<li>Full-day dates (like &quot;1 January 2018&quot; or &quot;tomorrow&quot;)</li>
<li>Floating due dates with time (like &quot;1 January 2018 at 12:00&quot; or &quot;tomorrow
  at 10am&quot;)</li>
<li>Due dates with time and fixed timezone (like &quot;1 January 2018 at 12:00
  America/Chicago&quot; or &quot;tomorrow at 10am Asia/Jakarta&quot;)</li>
</ul>
<p>Unless specified explicitly, dates with time are created as floating.</p>
<p>In addition, any of these due dates can be set to recurring or not, depending
on the date string, provided by the client.</p>
<p>Our Help Center contains an in-depth article about the difference between
<a href="https://www.todoist.com/help/articles/set-a-fixed-time-or-floating-time-for-a-task-YUYVp27q">floating due dates and dates with fixed zones</a>.</p>
<p>You can also find more information about
<a href="https://www.todoist.com/help/articles/introduction-to-recurring-due-dates-YUYVJJAV">recurring due dates</a> in our Help Center.</p>

<blockquote>
<p>Example full-day date:</p>
</blockquote>
<pre><code class="language-json"><span class="token punctuation">{</span>
    <span class="token string-property property">"date"</span><span class="token operator">:</span> <span class="token string">"2016-12-01"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"timezone"</span><span class="token operator">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
    <span class="token string-property property">"string"</span><span class="token operator">:</span> <span class="token string">"every day"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"lang"</span><span class="token operator">:</span> <span class="token string">"en"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"is_recurring"</span><span class="token operator">:</span> <span class="token boolean">true</span>
<span class="token punctuation">}</span>
</code></pre>
<h4 id="properties">Properties</h4>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>date <em>string</em></td>
<td>Due date in the format of <code>YYYY-MM-DD</code> (<a href="https://datatracker.ietf.org/doc/html/rfc3339">RFC 3339</a>). For recurring dates, the date of the current iteration.</td>
</tr>
<tr>
<td>timezone <em>string</em></td>
<td>Always set to <code>null</code>.</td>
</tr>
<tr>
<td>string <em>string</em></td>
<td>Human-readable representation of due date. String always represents the due object in user&#39;s timezone. Look at our reference to see <a href="https://www.todoist.com/help/articles/introduction-to-due-dates-and-due-times-q7VobO">which formats are supported</a>.</td>
</tr>
<tr>
<td>lang <em>string</em></td>
<td>Lang which has to be used to parse the content of the string attribute. Used by clients and on the server side to properly process due dates when date object is not set, and when dealing with recurring tasks. Valid languages are: <code>en</code>, <code>da</code>, <code>pl</code>, <code>zh</code>, <code>ko</code>, <code>de</code>, <code>pt</code>, <code>ja</code>, <code>it</code>, <code>fr</code>, <code>sv</code>, <code>ru</code>, <code>es</code>, <code>nl</code>, <code>fi</code>, <code>nb</code>, <code>tw</code>.</td>
</tr>
<tr>
<td>is<em>recurring _boolean</em></td>
<td>Boolean flag which is set to <code>true</code> if the due object represents a recurring due date.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example floating due date with time:</p>
</blockquote>
<pre><code class="language-json"><span class="token punctuation">{</span>
    <span class="token string-property property">"date"</span><span class="token operator">:</span> <span class="token string">"2016-12-0T12:00:00.000000"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"timezone"</span><span class="token operator">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
    <span class="token string-property property">"string"</span><span class="token operator">:</span> <span class="token string">"every day at 12"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"lang"</span><span class="token operator">:</span> <span class="token string">"en"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"is_recurring"</span><span class="token operator">:</span> <span class="token boolean">true</span>
<span class="token punctuation">}</span>
</code></pre>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>date <em>string</em></td>
<td>Due date in the format of <code>YYYY-MM-DDTHH:MM:SS</code>. For recurring dates, the date of the current iteration. Due date always represent an event in current user&#39;s timezone. Note that it&#39;s not quite compatible with <a href="https://datatracker.ietf.org/doc/html/rfc3339">RFC 3339</a>, because the concept of timezone is not applicable to this object. Also note that unlike fixed due dates, the date representation doesn&#39;t end with &quot;Z&quot;.</td>
</tr>
<tr>
<td>timezone <em>string</em></td>
<td>Always set to <code>null</code>.</td>
</tr>
<tr>
<td>string <em>string</em></td>
<td>Human-readable representation of due date. String always represents the due object in user&#39;s timezone. Look at our reference to see <a href="https://www.todoist.com/help/articles/introduction-to-due-dates-and-due-times-q7VobO">which formats are supported</a>.</td>
</tr>
<tr>
<td>lang <em>string</em></td>
<td>Lang which has to be used to parse the content of the string attribute. Used by clients and on the server side to properly process due dates when date object is not set, and when dealing with recurring tasks. Valid languages are: <code>en</code>, <code>da</code>, <code>pl</code>, <code>zh</code>, <code>ko</code>, <code>de</code>, <code>pt</code>, <code>ja</code>, <code>it</code>, <code>fr</code>, <code>sv</code>, <code>ru</code>, <code>es</code>, <code>nl</code>, <code>fi</code>, <code>nb</code>, <code>tw</code>.</td>
</tr>
<tr>
<td>is<em>recurring _boolean</em></td>
<td>Boolean flag which is set to <code>true</code> if the due object represents a recurring due date.</td>
</tr>
</tbody></table>

<blockquote>
<p>Example due date with time and fixed timezone:</p>
</blockquote>
<pre><code class="language-json"><span class="token punctuation">{</span>
    <span class="token string-property property">"date"</span><span class="token operator">:</span> <span class="token string">"2016-12-06T13:00:00.000000Z"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"timezone"</span><span class="token operator">:</span> <span class="token string">"Europe/Madrid"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"string"</span><span class="token operator">:</span> <span class="token string">"ev day at 2pm"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"lang"</span><span class="token operator">:</span> <span class="token string">"en"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"is_recurring"</span><span class="token operator">:</span> <span class="token boolean">true</span>
<span class="token punctuation">}</span>
</code></pre>
<h4 id="properties">Properties</h4>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>date <em>string</em></td>
<td>Due date in the format of <code>YYYY-MM-DDTHH:MM:SSZ</code> (<a href="https://datatracker.ietf.org/doc/html/rfc3339">RFC 3339</a>). For recurring dates, the date of the current iteration. Due date is stored in UTC.</td>
</tr>
<tr>
<td>timezone <em>string</em></td>
<td>Timezone of the due instance. Used to recalculate properly the next iteration for a recurring due date.</td>
</tr>
<tr>
<td>string <em>string</em></td>
<td>Human-readable representation of due date. String always represents the due object in user&#39;s timezone. Look at our reference to see <a href="https://www.todoist.com/help/articles/introduction-to-due-dates-and-due-times-q7VobO">which formats are supported</a>.</td>
</tr>
<tr>
<td>lang <em>string</em></td>
<td>Lang which has to be used to parse the content of the string attribute. Used by clients and on the server side to properly process due dates when date object is not set, and when dealing with recurring tasks. Valid languages are: <code>en</code>, <code>da</code>, <code>pl</code>, <code>zh</code>, <code>ko</code>, <code>de</code>, <code>pt</code>, <code>ja</code>, <code>it</code>, <code>fr</code>, <code>sv</code>, <code>ru</code>, <code>es</code>, <code>nl</code>, <code>fi</code>, <code>nb</code>, <code>tw</code>.</td>
</tr>
<tr>
<td>is<em>recurring _boolean</em></td>
<td>Boolean flag which is set to <code>true</code> is due object represents a recurring due date</td>
</tr>
</tbody></table>

<p>Usually you create due dates when you create a new task or a reminder, or
you want to update a due date for an object. In both cases due date is provided
as a <code>due</code> attribute of an object. You may provide all fields of an object in
the constructor, but it&#39;s more convenient to provide only a subset of the
fields and let the server fill the gaps.</p>
<h4 id="create-or-update-due-date-from-user-provided-string">Create or update due date from user-provided string</h4>
<blockquote>
<p>Input example</p>
</blockquote>
<pre><code class="language-json"><span class="token string-property property">"due"</span><span class="token operator">:</span> <span class="token punctuation">{</span><span class="token string-property property">"string"</span><span class="token operator">:</span>  <span class="token string">"tomorrow"</span><span class="token punctuation">}</span>
</code></pre>
<blockquote>
<p>Output example. Full-date instance is created.</p>
</blockquote>
<pre><code class="language-json"><span class="token string-property property">"due"</span><span class="token operator">:</span> <span class="token punctuation">{</span>
    <span class="token string-property property">"date"</span><span class="token operator">:</span> <span class="token string">"2018-11-15"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"timezone"</span><span class="token operator">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
    <span class="token string-property property">"is_recurring"</span><span class="token operator">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string-property property">"string"</span><span class="token operator">:</span> <span class="token string">"tomorrow"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"lang"</span><span class="token operator">:</span> <span class="token string">"en"</span>
<span class="token punctuation">}</span>
</code></pre>
<blockquote>
<p>Input example</p>
</blockquote>
<pre><code class="language-json"><span class="token string-property property">"due"</span><span class="token operator">:</span> <span class="token punctuation">{</span><span class="token string-property property">"string"</span><span class="token operator">:</span>  <span class="token string">"tomorrow at 12"</span><span class="token punctuation">}</span>
</code></pre>
<blockquote>
<p>Output example. Floating due date created</p>
</blockquote>
<pre><code class="language-json"><span class="token string-property property">"due"</span><span class="token operator">:</span> <span class="token punctuation">{</span>
    <span class="token string-property property">"date"</span><span class="token operator">:</span> <span class="token string">"2018-11-15T12:00:00.000000"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"timezone"</span><span class="token operator">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
    <span class="token string-property property">"is_recurring"</span><span class="token operator">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string-property property">"string"</span><span class="token operator">:</span> <span class="token string">"tomorrow at 12"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"lang"</span><span class="token operator">:</span> <span class="token string">"en"</span>
<span class="token punctuation">}</span>
</code></pre>
<blockquote>
<p>Input example. Timezone is set explicitly</p>
</blockquote>
<pre><code class="language-json"><span class="token string-property property">"due"</span><span class="token operator">:</span> <span class="token punctuation">{</span><span class="token string-property property">"string"</span><span class="token operator">:</span> <span class="token string">"tomorrow at 12"</span><span class="token punctuation">,</span> <span class="token string-property property">"timezone"</span><span class="token operator">:</span> <span class="token string">"Asia/Jakarta"</span><span class="token punctuation">}</span>
</code></pre>
<blockquote>
<p>Output example. Due date with fixed timezone created</p>
</blockquote>
<pre><code class="language-json"><span class="token string-property property">"due"</span><span class="token operator">:</span> <span class="token punctuation">{</span>
    <span class="token string-property property">"date"</span><span class="token operator">:</span> <span class="token string">"2018-11-16T05:00:00.000000Z"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"timezone"</span><span class="token operator">:</span> <span class="token string">"Asia/Jakarta"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"is_recurring"</span><span class="token operator">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string-property property">"string"</span><span class="token operator">:</span> <span class="token string">"tomorrow at 12"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"lang"</span><span class="token operator">:</span> <span class="token string">"en"</span>
<span class="token punctuation">}</span>
</code></pre>
<p>You can ask the user to provide a due string and to create a new object from that.
You need to provide a timezone if you want to create a fixed due date instead
of a floating one. If you want to create a task without a due date, you
can set the due attribute to <code>null</code>.</p>
<p>See the code section to the right for more examples. In all cases you can set
the <code>lang</code> attribute of the date to set the language of the input. If the language
is not set, the language from user settings will be used.</p>
<h4 id="create-or-update-due-date-from-a-date-object">Create or update due date from a date object</h4>
<blockquote>
<p>Input example for a full-day event</p>
</blockquote>
<pre><code class="language-json"><span class="token string-property property">"due"</span><span class="token operator">:</span> <span class="token punctuation">{</span><span class="token string-property property">"date"</span><span class="token operator">:</span> <span class="token string">"2018-10-14"</span><span class="token punctuation">}</span>
</code></pre>
<p>For a full-day event the format of the date attribute is <code>YYYY-MM-DD</code>.</p>
<blockquote>
<p>Output example</p>
</blockquote>
<pre><code class="language-json"><span class="token string-property property">"due"</span><span class="token operator">:</span> <span class="token punctuation">{</span>
    <span class="token string-property property">"date"</span><span class="token operator">:</span> <span class="token string">"2018-10-14"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"timezone"</span><span class="token operator">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
    <span class="token string-property property">"is_recurring"</span><span class="token operator">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string-property property">"string"</span><span class="token operator">:</span> <span class="token string">"2018-10-14"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"lang"</span><span class="token operator">:</span> <span class="token string">"en"</span>
<span class="token punctuation">}</span>
</code></pre>
<blockquote>
<p>Input example for a floating due date</p>
</blockquote>
<pre><code class="language-json"><span class="token string-property property">"due"</span><span class="token operator">:</span> <span class="token punctuation">{</span><span class="token string-property property">"date"</span><span class="token operator">:</span> <span class="token string">"2018-10-14T10:00:00.000000"</span><span class="token punctuation">}</span>
</code></pre>
<blockquote>
<p>Output example</p>
</blockquote>
<pre><code class="language-json"><span class="token string-property property">"due"</span><span class="token operator">:</span> <span class="token punctuation">{</span>
    <span class="token string-property property">"date"</span><span class="token operator">:</span> <span class="token string">"2018-10-14T10:00:00.000000"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"timezone"</span><span class="token operator">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
    <span class="token string-property property">"is_recurring"</span><span class="token operator">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string-property property">"string"</span><span class="token operator">:</span> <span class="token string">"2018-10-14 10:00"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"lang"</span><span class="token operator">:</span> <span class="token string">"en"</span>
<span class="token punctuation">}</span>
</code></pre>
<p>In some cases you have a date object and want to create a due date from it.
Usually all you need to do is choose the format of the due date (floating
or fixed) and format the time object properly with strftime or alternative for
your programming language. The formatted string goes to a &quot;date&quot; attribute of
the constructor.</p>
<p>Note that this approach does not allow you to create recurring due dates.</p>
<p>For a floating due date event the format of the date attribute is
<code>YYYY-MM-DDTHH:MM:SS</code> and the date has to be provided in user&#39;s local
timezone.</p>
<blockquote>
<p>Input example for a due date with a fixed timezone</p>
</blockquote>
<pre><code class="language-json"><span class="token string-property property">"due"</span><span class="token operator">:</span> <span class="token punctuation">{</span><span class="token string-property property">"date"</span><span class="token operator">:</span> <span class="token string">"2018-10-14T05:00:00.000000Z"</span><span class="token punctuation">}</span>
</code></pre>
<blockquote>
<p>Output example</p>
</blockquote>
<pre><code class="language-json"><span class="token string-property property">"due"</span><span class="token operator">:</span> <span class="token punctuation">{</span>
    <span class="token string-property property">"date"</span><span class="token operator">:</span> <span class="token string">"2018-10-14T05:00:00.000000Z"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"timezone"</span><span class="token operator">:</span> <span class="token string">"Asia/Jakarta"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"is_recurring"</span><span class="token operator">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
    <span class="token string-property property">"string"</span><span class="token operator">:</span> <span class="token string">"2018-10-14 12:00"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"lang"</span><span class="token operator">:</span> <span class="token string">"en"</span>
<span class="token punctuation">}</span>
</code></pre>
<p>For a fixed due date event the format of the date attribute is
<code>YYYY-MM-DDTHH:MM:SSZ</code> (note the &quot;Z&quot; ending) and the date has to be provided
in UTC. Optionally you can provide a timezone name to overwrite the default
timezone of the user.</p>

<p>Similar to due dates, deadlines can be set on tasks, and can be used to differentiate
between when a task should be started, and when it must be done by.</p>
<p>Unlike due dates, deadlines only support non-recurring dates with no time component.</p>
<p>You can find our more information about
<a href="https://www.todoist.com/help/articles/introduction-to-deadlines-uMqbSLM6U">deadlines</a> in
our Help Center.</p>

<pre><code class="language-json"><span class="token punctuation">{</span>
    <span class="token string-property property">"date"</span><span class="token operator">:</span> <span class="token string">"2016-12-01"</span>
<span class="token punctuation">}</span>
</code></pre>
<h4 id="properties">Properties</h4>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>date <em>string</em></td>
<td>Deadline in the format of <code>YYYY-MM-DD</code> (<a href="https://datatracker.ietf.org/doc/html/rfc3339">RFC 3339</a>).</td>
</tr>
<tr>
<td>lang <em>string</em></td>
<td>Only returned on the output, for future compatibility reasons. Currently unused in the processing of the <code>date</code> property. Possible values are: <code>en</code>, <code>da</code>, <code>pl</code>, <code>zh</code>, <code>ko</code>, <code>de</code>, <code>pt</code>, <code>ja</code>, <code>it</code>, <code>fr</code>, <code>sv</code>, <code>ru</code>, <code>es</code>, <code>nl</code>, <code>fi</code>, <code>nb</code>, <code>tw</code>.</td>
</tr>
</tbody></table>

<p>Usually you create deadlines when you create a new task, or  you want to update a
deadline for an object. In both cases due date is provided as a <code>deadline</code> attribute of
an object.</p>
<h4 id="create-or-update-deadline">Create or update deadline</h4>
<blockquote>
<p>Input example</p>
</blockquote>
<pre><code class="language-json"><span class="token string-property property">"deadline"</span><span class="token operator">:</span> <span class="token punctuation">{</span><span class="token string-property property">"date"</span><span class="token operator">:</span>  <span class="token string">"2024-01-25"</span><span class="token punctuation">}</span>
</code></pre>
<blockquote>
<p>Output example</p>
</blockquote>
<pre><code class="language-json"><span class="token string-property property">"deadline"</span><span class="token operator">:</span> <span class="token punctuation">{</span>
    <span class="token string-property property">"date"</span><span class="token operator">:</span> <span class="token string">"2024-01-25"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"lang"</span><span class="token operator">:</span> <span class="token string">"en"</span>
<span class="token punctuation">}</span>
</code></pre>

<p>Get information about the currently authenticated user.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Get comprehensive productivity statistics for the authenticated user.</p>
<p>Returns detailed completion statistics including:</p>
<ul>
<li>Daily completion counts with per-project breakdowns for the last 7 days</li>
<li>Weekly completion counts with per-project breakdowns for the last 4 weeks</li>
<li>Total completed task count</li>
<li>Karma score, trend, graph data, and update history</li>
<li>Goal settings (daily/weekly goals, ignore days, vacation mode)</li>
<li>Streak information (current, last, and maximum daily and weekly streaks)</li>
<li>Project color mappings for visualization</li>
</ul>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>The type of notification being sent</p>

<p>Which communication mechanism is being used to send this notification</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p><em>Availability of the activity log and the duration of event storage are
dependent on the current user plan. These values are indicated by the
<code>activity_log</code> and <code>activity_log_limit</code> properties of the <a href="#tag/Sync/User/User-plan-limits">user plan
limits</a> object.</em></p>
<p>The activity log makes it easy to see everything that is happening across projects, items and notes.</p>
<p><strong>Note:</strong> The activity log uses a unique page-based pagination system that differs from the standard cursor-based pagination used by most other endpoints. For information about cursor-based pagination, see the <a href="#tag/Pagination">Pagination guide</a>.</p>
<h3 id="logged-events">Logged events</h3>
<p>Currently the official Todoist clients present only the most important events
that most users may be interested in.
There are further types of events related to projects, items and notes that are
stored in our database, and can be accessed through the API.</p>
<p>The following events are logged for items:</p>
<ul>
<li>Items added</li>
<li>Items updated (only changes to <code>content</code>, <code>description</code>, <code>due_date</code> and <code>responsible_uid</code>)</li>
<li>Items deleted</li>
<li>Items completed</li>
<li>Items uncompleted</li>
</ul>
<p>The following events are logged for notes:</p>
<ul>
<li>Notes added</li>
<li>Notes updated (only changes to <code>content</code> or <code>file_name</code> if the former is empty)</li>
<li>Notes deleted</li>
</ul>
<p>The following events are logged for projects:</p>
<ul>
<li>Projects added</li>
<li>Projects updated (only changes to <code>name</code>)</li>
<li>Projects deleted</li>
<li>Projects archived</li>
<li>Projects unarchived</li>
<li>Projects shared</li>
<li>Projects left</li>
</ul>
<h3 id="pagination-details">Pagination details</h3>
<p>There are 3 parameters that control which events are returned from the activity
log. These parameters should be used in combination to get all the events one
is interested in.</p>
<h4 id="the-page-parameter">The <code>page</code> parameter</h4>
<p>The events in the activity log are organized by week. Each week starts at
Sunday <code>12:00:00</code> (PM or noon), and ends the next Sunday at <code>11:59:59</code>, This
means that one can target a specific week, and get events from that week. The
<code>page</code> parameter specifies from which week to fetch events, and it does so in a
way that is relative to the current time.</p>
<p>This will be more easy to understand with the following example. Assuming it&#39;s
now <code>Wednesday, February 23</code>, then:</p>
<ul>
<li><code>page=0</code>: Denotes events from the current week, that is from <code>Sunday, February 20</code>, to just now</li>
<li><code>page=1</code>: Denotes events from last week, from <code>February 13</code>, to <code>February 20</code></li>
<li><code>page=2</code>: Denotes events from 2 weeks ago, from <code>February 6</code>, to <code>February 13</code></li>
<li><code>page=3</code>: Denotes events from 3 weeks ago, from <code>January 30</code>, to <code>February 6</code></li>
</ul>
<p>And so on.</p>
<p>If the <code>page</code> parameter is not specified, then events from the current and last
week are returned. This is equivalent to getting events for <code>page=0</code> and
<code>page=1</code> together. So omitting the <code>page</code> parameter, and depending on which day
of the week the call is made, this should return events from <code>7</code> to <code>14</code> days
ago. This is useful in order to always fetch at least a week&#39;s events, even on
Mondays.</p>
<p>In the above example, this would return events from <code>Sunday, February 13</code> to
<code>Wednesday, February 23</code>, so around <code>10</code> days.</p>
<h4 id="the-limit-and-offset-parameters">The <code>limit</code> and <code>offset</code> parameters</h4>
<p>Each week can have a lot of events. This is where the <code>limit</code> and <code>offset</code>
parameters come into play. Because it&#39;s not resource friendly to get hundreds
of events in one call, the events returned are limited by the default value of
the <code>limit</code> parameter, as defined above in the <a href="#tag/Activity">Properties</a>
section. This limit can be increased, but up to a maximum value, again defined
in the <a href="#tag/Activity">Properties</a> section.</p>
<p>Since not all of the events of a specific week, can be returned in a single
call, a subsequent call should use the <code>offset</code> parameter, in order to skip the
events already received.</p>
<p>As an example, assuming that the current week (ie. <code>page=0</code>) has <code>78</code> events,
and that a <code>limit=50</code> is used in order to get up to <code>50</code> events in each call,
one would need to do 2 calls:</p>
<ol>
<li>A request with parameters <code>page=0</code>, <code>limit=50</code>, and <code>offset=0</code>, will return <code>50</code> events and also the <code>count=78</code> value</li>
<li>Since the return value <code>count=78</code> is larger than <code>limit=50</code>, an additional call is needed with the parameters <code>page=0</code>, <code>limit=50</code>, and <code>offset=50</code>, which will return the rest of the <code>28</code> events</li>
</ol>
<p>If last week had <code>234</code> events, and assuming a <code>limit=100</code> was used:</p>
<ol>
<li>A request with <code>page=1</code>, <code>limit=100</code> and <code>offset=0</code>, will return <code>100</code> events, and <code>count=234</code></li>
<li>A second request with <code>page=1</code>, <code>limit=100</code> and <code>offset=100</code>, will return additional <code>100</code> events</li>
<li>A third request with <code>page=1</code>, <code>limit=100</code> and <code>offset=200</code>, will return the remaining <code>34</code> events</li>
</ol>

<p>Get activity logs.</p>
<p>Returns a paginated list of activity events for the user. Events can be filtered by object
type (project, item, note), event type, and other criteria. Uses cursor-based pagination
for efficient navigation through results.</p>

<p>The type of object to filter activities by. Must be one of &quot;project&quot;, &quot;item&quot; (task), or &quot;note&quot; (comment). When specified with <code>object_id</code>, returns activities for that specific object.</p>

<p>The ID of the specific object to get activities for. Must be used together with <code>object_type</code>. For example, to get activities for a specific task, set <code>object_type=item</code> and <code>object_id=&lt;task_id&gt;</code>.</p>

<p>Filter activities to only those belonging to the specified project. Returns activities for the project itself and all its tasks and comments.</p>

<p>Filter activities to only those belonging to the specified task. Returns activities for the task itself and all its comments.</p>

<p>When <code>true</code> and <code>object_id</code> is specified, also include activities for the parent object. For example, when filtering by a specific task, also include activities for its parent project.</p>

<p>When <code>true</code> and <code>object_id</code> is specified, also include activities for all child objects. For example, when filtering by a project, also include activities for all its tasks and comments.</p>

<p>Filter activities to only those initiated by the specified user ID(s). Accepts either a single user ID or a list of user IDs. Useful for shared projects to see who made which changes.</p>

<p>Filter by whether the activity has an initiator. When <code>true</code>, returns only activities with no initiator (your own activities). When <code>false</code>, returns only activities initiated by collaborators.</p>

<p>Filter by a simple event type (e.g., &quot;added&quot;, &quot;deleted&quot;, &quot;completed&quot;). Returns events of this type across ALL object types that support it. For more precise filtering by both object type and event type, use <code>object_event_types</code> instead.</p>

<p><strong>Deprecated</strong> - This parameter has no implementation and will be removed in a future version.</p>

<p>Advanced filtering for specific object type and event type combinations. Format: <code>[&quot;object_type:event_type&quot;]</code>. Examples: <code>[&quot;item:deleted&quot;]</code> for deleted tasks, <code>[&quot;item:&quot;]</code> for all task events, <code>[&quot;:deleted&quot;]</code> for all delete events across all types, <code>[&quot;item:deleted&quot;, &quot;note:added&quot;]</code> for multiple filters. Valid event types: &quot;added&quot;, &quot;deleted&quot;, &quot;updated&quot;, &quot;completed&quot;, &quot;uncompleted&quot;, &quot;archived&quot;, &quot;unarchived&quot;, &quot;shared&quot;, &quot;left&quot;, &quot;reordered&quot;, &quot;moved&quot;. This is the recommended way to filter events.</p>

<p>Filter activities to only those belonging to the specified workspace(s). Accepts either a single workspace ID or a list of workspace IDs.</p>

<p>When <code>true</code>, includes additional information about comments in the <code>extra_data</code> field, such as the content of the comment.</p>

<p>When <code>true</code>, includes additional information about parent objects in the <code>extra_data</code> field, such as the name of the parent project or task.</p>

<p>Pagination cursor for fetching the next page of results. Use the value returned in the <code>next_cursor</code> field from a previous response.</p>

<p>Maximum number of activity logs to return per page.</p>

<p>Filter activities to only those that occurred on or after this date. Must be in ISO 8601 format (e.g. &#39;2026-01-01T00:00:00Z&#39;). When specified, overrides the default pagination behavior and allows custom date ranges.</p>

<p>Filter activities to only those that occurred before this date (exclusive upper bound). Must be in ISO 8601 format (e.g. &#39;2026-02-01T00:00:00Z&#39;). When specified, overrides the default pagination behavior and allows custom date ranges.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p><em>Availability of backups functionality is dependent on the current user plan. This value is indicated by the automatic_backups property of the user plan limits object.</em></p>

<p>Backup URL</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Todoist creates a backup archive of users&#39; data on a daily basis. Backup
archives can also be accessed from the web app (Todoist Settings -&gt;
Backups).</p>
<p>When using the default token, with the <code>data:read_write</code> scope, and having MFA enabled, the MFA
token is required and must be provided with the request. To be able to use this endpoint without an
MFA token, your token must have the <code>backups:read</code> scope.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Disable the current email to a Todoist object</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>Get or create an email to a Todoist object,
currently only projects and tasks are supported.</p>

<p>Successful Response</p>

<p>Bad Request</p>

<p>Unauthorized</p>

<p>Forbidden</p>

<p>Not Found</p>

<p>The Todoist Webhooks API allows applications to receive real-time notification
(in the form of HTTP POST payload) on the subscribed user events. Notice that
once you have a webhook setup, you will start receiving webhook events from
<strong>all your app users</strong> immediately.</p>
<h4 id="important-considerations">Important Considerations</h4>
<ul>
<li><p>For security reasons, Todoist only allows webhook urls that have HTTPS enabled and no ports specified in the url.</p>
<ul>
<li>Allowed webhook url:<ul>
<li><code>https://nice.integration.com</code></li>
</ul>
</li>
<li>Disallowed webhook url:<ul>
<li><code>http://evil.integration.com</code></li>
<li><code>https://bad.integration.com:5980</code></li>
</ul>
</li>
</ul>
</li>
<li><p>Due to the nature of network requests, your application should assume webhook
requests could arrive delayed, out of order, or could even fail to arrive at all; webhooks should
be used only as notifications and not as primary Todoist data sources (make
sure your application could still work when webhook is not available).</p>
</li>
</ul>
<h4 id="webhook-activation--personal-use">Webhook Activation &amp; Personal Use</h4>
<p>The webhook for a specific user is activated when that user completes the <a href="#tag/Authorization/OAuth">OAuth flow</a> of the app that declares the webhook.</p>
<p><strong>Todoist webhooks don&#39;t fire by default for the user that has created the Todoist app, which is frequently the desired state for the personal use of webhooks.</strong></p>
<p>To activate webhooks for personal use, you need to complete the OAuth process with your account. You can do this without code by manually executing the OAuth flow in two steps.</p>
<ol>
<li>Performing the <a href="#tag/Authorization/OAuth">authorization request</a> in the browser and capturing the <code>code</code> via the browser&#39;s developer tools.</li>
<li>Performing the <a href="#tag/Authorization/OAuth">token exchange request</a> through a tool like <a href="https://www.postman.com/">Postman</a> and reading the <code>access_token</code> from the response. <em>Note that you can&#39;t make this request via the browser as it needs to be a POST request.</em></li>
</ol>

<p>Before you can start receiving webhook event notifications, you must first have
your webhook configured at the App Management Console.</p>
<h4 id="events">Events</h4>
<p>Here is a list of events that you could subscribe to, and they are configured at
the <a href="https://app.todoist.com/app/settings/integrations/app-management">App Management Console</a>.</p>
<table>
<thead>
<tr>
<th>Event Name</th>
<th>Description</th>
<th>Event Data</th>
</tr>
</thead>
<tbody><tr>
<td>item:added</td>
<td>A task was added</td>
<td>The new <a href="#tag/Sync/Tasks">Task</a>.</td>
</tr>
<tr>
<td>item:updated</td>
<td>A task was updated</td>
<td>The updated <a href="#tag/Sync/Tasks">Task</a>.</td>
</tr>
<tr>
<td>item:deleted</td>
<td>A task was deleted</td>
<td>The deleted <a href="#tag/Sync/Tasks">Task</a>.</td>
</tr>
<tr>
<td>item:completed</td>
<td>A task was completed</td>
<td>The completed <a href="#tag/Sync/Tasks">Task</a>.</td>
</tr>
<tr>
<td>item:uncompleted</td>
<td>A task was uncompleted</td>
<td>The uncompleted <a href="#tag/Sync/Tasks">Task</a>.</td>
</tr>
<tr>
<td>note:added</td>
<td>A comment was added</td>
<td>The new <a href="#tag/Sync/Comments">Comment</a>.</td>
</tr>
<tr>
<td>note:updated</td>
<td>A comment was updated</td>
<td>The updated <a href="#tag/Sync/Comments">Comment</a>.</td>
</tr>
<tr>
<td>note:deleted</td>
<td>A comment was deleted</td>
<td>The deleted <a href="#tag/Sync/Comments">Comment</a>.</td>
</tr>
<tr>
<td>project:added</td>
<td>A project was added</td>
<td>The new <a href="#tag/Sync/Projects">Project</a>.</td>
</tr>
<tr>
<td>project:updated</td>
<td>A project was updated</td>
<td>The updated <a href="#tag/Sync/Projects">Project</a>.</td>
</tr>
<tr>
<td>project:deleted</td>
<td>A project was deleted</td>
<td>The deleted <a href="#tag/Sync/Projects">Project</a>.</td>
</tr>
<tr>
<td>project:archived</td>
<td>A project was archived</td>
<td>The archived <a href="#tag/Sync/Projects">Project</a>.</td>
</tr>
<tr>
<td>project:unarchived</td>
<td>A project was unarchived</td>
<td>The unarchived <a href="#tag/Sync/Projects">Project</a>.</td>
</tr>
<tr>
<td>section:added</td>
<td>A section was added</td>
<td>The new <a href="#tag/Sync/Sections">Section</a>.</td>
</tr>
<tr>
<td>section:updated</td>
<td>A section was updated</td>
<td>The updated <a href="#tag/Sync/Sections">Section</a>.</td>
</tr>
<tr>
<td>section:deleted</td>
<td>A section was deleted</td>
<td>The deleted <a href="#tag/Sync/Sections">Section</a>.</td>
</tr>
<tr>
<td>section:archived</td>
<td>A section was archived</td>
<td>The archived <a href="#tag/Sync/Sections">Section</a>.</td>
</tr>
<tr>
<td>section:unarchived</td>
<td>A section was unarchived</td>
<td>The unarchived <a href="#tag/Sync/Sections">Section</a>.</td>
</tr>
<tr>
<td>label:added</td>
<td>A label was added</td>
<td>The new <a href="#tag/Sync/Labels">Label</a>.</td>
</tr>
<tr>
<td>label:deleted</td>
<td>A label was deleted</td>
<td>The deleted <a href="#tag/Sync/Labels">Label</a>.</td>
</tr>
<tr>
<td>label:updated</td>
<td>A label was updated</td>
<td>The updated <a href="#tag/Sync/Labels">Label</a>.</td>
</tr>
<tr>
<td>filter:added</td>
<td>A filter was added</td>
<td>The new <a href="#tag/Sync/Filters">Filter</a>.</td>
</tr>
<tr>
<td>filter:deleted</td>
<td>A filter was deleted</td>
<td>The deleted <a href="#tag/Sync/Filters">Filter</a>.</td>
</tr>
<tr>
<td>filter:updated</td>
<td>A filter was updated</td>
<td>The updated <a href="#tag/Sync/Filters">Filter</a>.</td>
</tr>
<tr>
<td>reminder:fired</td>
<td>A reminder has fired</td>
<td>The <a href="#/tag/Sync/Reminders">Reminder</a> that fired.</td>
</tr>
</tbody></table>
<h4 id="events-extra">Events Extra</h4>
<p>Some events can include extra meta information in the <code>event_data_extra</code> field. These can be useful, for example, if you need to distinguish between item updates that are postponed and initiated by the user and item updates that are task completions (initiated by completing a recurring task)</p>
<table>
<thead>
<tr>
<th>Event Name</th>
<th>Description</th>
<th>Event Data</th>
</tr>
</thead>
<tbody><tr>
<td>item:updated</td>
<td>For events issued by the user directly these include <code>old_item</code> and <code>update_intent</code></td>
<td><code>old_item</code> will be an <a href="#tag/Sync/Tasks">Task</a>, and <code>update_intent</code> can be <code>item_updated</code>, <code>item_completed</code>, <code>item_uncompleted</code>.</td>
</tr>
</tbody></table>

<h4 id="event-json-object">Event JSON Object</h4>
<blockquote>
<p>Example Webhook Request</p>
</blockquote>
<pre><code class="language-text">POST /payload HTTP/1.1

Host: your_callback_url_host
Content-Type: application/json
X-Todoist-Hmac-SHA256: UEEq9si3Vf9yRSrLthbpazbb69kP9+CZQ7fXmVyjhPs=
</code></pre>
<pre><code class="language-json"><span class="token punctuation">{</span>
    <span class="token string-property property">"event_name"</span><span class="token operator">:</span> <span class="token string">"item:added"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"user_id"</span><span class="token operator">:</span> <span class="token string">"2671355"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"event_data"</span><span class="token operator">:</span> <span class="token punctuation">{</span>
        <span class="token string-property property">"added_by_uid"</span><span class="token operator">:</span> <span class="token string">"2671355"</span><span class="token punctuation">,</span>
        <span class="token string-property property">"assigned_by_uid"</span><span class="token operator">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
        <span class="token string-property property">"checked"</span><span class="token operator">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
        <span class="token string-property property">"child_order"</span><span class="token operator">:</span> <span class="token number">3</span><span class="token punctuation">,</span>
        <span class="token string-property property">"collapsed"</span><span class="token operator">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
        <span class="token string-property property">"content"</span><span class="token operator">:</span> <span class="token string">"Buy Milk"</span><span class="token punctuation">,</span>
        <span class="token string-property property">"description"</span><span class="token operator">:</span> <span class="token string">""</span><span class="token punctuation">,</span>
        <span class="token string-property property">"added_at"</span><span class="token operator">:</span> <span class="token string">"2025-02-10T10:33:38.000000Z"</span><span class="token punctuation">,</span>
        <span class="token string-property property">"completed_at"</span><span class="token operator">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
        <span class="token string-property property">"due"</span><span class="token operator">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
        <span class="token string-property property">"deadline"</span><span class="token operator">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
        <span class="token string-property property">"id"</span><span class="token operator">:</span> <span class="token string">"6XR4GqQQCW6Gv9h4"</span><span class="token punctuation">,</span>
        <span class="token string-property property">"is_deleted"</span><span class="token operator">:</span> <span class="token boolean">false</span><span class="token punctuation">,</span>
        <span class="token string-property property">"labels"</span><span class="token operator">:</span> <span class="token punctuation">[</span><span class="token punctuation">]</span><span class="token punctuation">,</span>
        <span class="token string-property property">"parent_id"</span><span class="token operator">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
        <span class="token string-property property">"priority"</span><span class="token operator">:</span> <span class="token number">1</span><span class="token punctuation">,</span>
        <span class="token string-property property">"project_id"</span><span class="token operator">:</span> <span class="token string">"6XR4H993xv8H5qCR"</span><span class="token punctuation">,</span>
        <span class="token string-property property">"responsible_uid"</span><span class="token operator">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
        <span class="token string-property property">"section_id"</span><span class="token operator">:</span> <span class="token keyword">null</span><span class="token punctuation">,</span>
        <span class="token string-property property">"url"</span><span class="token operator">:</span> <span class="token string">"https://app.todoist.com/app/task/6XR4GqQQCW6Gv9h4"</span><span class="token punctuation">,</span>
        <span class="token string-property property">"user_id"</span><span class="token operator">:</span> <span class="token string">"2671355"</span>
    <span class="token punctuation">}</span><span class="token punctuation">,</span>
    <span class="token string-property property">"initiator"</span><span class="token operator">:</span> <span class="token punctuation">{</span>
        <span class="token string-property property">"email"</span><span class="token operator">:</span> <span class="token string">"alice@example.com"</span><span class="token punctuation">,</span>
        <span class="token string-property property">"full_name"</span><span class="token operator">:</span> <span class="token string">"Alice"</span><span class="token punctuation">,</span>
        <span class="token string-property property">"id"</span><span class="token operator">:</span> <span class="token string">"2671355"</span><span class="token punctuation">,</span>
        <span class="token string-property property">"image_id"</span><span class="token operator">:</span> <span class="token string">"ad38375bdb094286af59f1eab36d8f20"</span><span class="token punctuation">,</span>
        <span class="token string-property property">"is_premium"</span><span class="token operator">:</span> <span class="token boolean">true</span>
    <span class="token punctuation">}</span><span class="token punctuation">,</span>
    <span class="token string-property property">"triggered_at"</span><span class="token operator">:</span> <span class="token string">"2025-02-10T10:39:38.000000Z"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"version"</span><span class="token operator">:</span> <span class="token string">"10"</span>
<span class="token punctuation">}</span>
</code></pre>
<p>Each webhook event notification request contains a JSON object. The event JSON
contains the following properties:</p>
<table>
<thead>
<tr>
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>event_name <em>String</em></td>
<td>The event name for the webhook, see the table in the <a href="#tag/Webhooks/Configuration">Configuration</a> section for the list of supported events.</td>
</tr>
<tr>
<td>user_id <em>String</em></td>
<td>The ID of the user that is the destination for the event.</td>
</tr>
<tr>
<td>event_data <em>Object</em></td>
<td>An object representing the modified entity that triggered the event, see the table in the <a href="#tag/Webhooks/Configuration">Configuration</a> section for details of the <code>event_data</code> for each event.</td>
</tr>
<tr>
<td>version <em>String</em></td>
<td>The version number of the webhook configured in the <a href="https://app.todoist.com/app/settings/integrations/app-management">App Management Console</a>.</td>
</tr>
<tr>
<td>initiator <em>Object</em></td>
<td>A <a href="#collaborators">Collaborator</a> object representing the user that triggered the event. This may be the same user indicated in <code>user_id</code> or a collaborator from a shared project.</td>
</tr>
<tr>
<td>triggered_at <em>String</em></td>
<td>The date and time when the event was triggered.</td>
</tr>
<tr>
<td>event_data_extra <em>Object</em></td>
<td>Optional object that can include meta information, see the table in the <a href="#tag/Webhooks/Configuration">Configuration</a> section for details of the <code>event_data_extra</code> for each event.</td>
</tr>
</tbody></table>
<h4 id="request-header">Request Header</h4>
<table>
<thead>
<tr>
<th>Header Name</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>User-Agent</td>
<td>Will be set to &quot;Todoist-Webhooks&quot;</td>
</tr>
<tr>
<td>X-Todoist-Hmac-SHA256</td>
<td>To verify each webhook request was indeed sent by Todoist, an <code>X-Todoist-Hmac-SHA256</code> header is included; it is a SHA256 Hmac generated using your <code>client_secret</code> as the encryption key and the whole request payload as the message to be encrypted. The resulting Hmac would be encoded in a base64 string.</td>
</tr>
<tr>
<td>X-Todoist-Delivery-ID</td>
<td>Each webhook event notification has a unique <code>X-Todoist-Delivery-ID</code>. When a notification request failed to be delivered to your endpoint, the request would be re-delivered with the same <code>X-Todoist-Delivery-ID</code>.</td>
</tr>
</tbody></table>
<h4 id="failed-delivery">Failed Delivery</h4>
<p>When an event notification fails to be delivered to your webhook callback URL
(i.e. due to server / network error, incorrect response, etc),
it will be reattempted after 15 minutes. Each notification will be
reattempted for at most three times.</p>
<p><strong>Your callback endpoint must respond with an HTTP 200 when receiving an event
notification request.</strong></p>
<p>A response other than HTTP 200 will be considered as a failed delivery, and the
notification will be attempted again.</p>

<p>Many endpoints in the Todoist API return paginated results to handle large datasets efficiently. This guide explains how pagination works and how to use it effectively.</p>

<p>Paginated endpoints use <strong>cursor-based pagination</strong>. Instead of using page numbers or offsets, you use an opaque cursor token to retrieve the next set of results.</p>
<h3 id="response-format">Response Format</h3>
<p>Paginated endpoints return a response with two key fields:</p>
<ul>
<li><code>results</code>: An array containing the requested objects</li>
<li><code>next_cursor</code>: A string token for fetching the next page, or <code>null</code> if there are no more results</li>
</ul>
<p>Example response:</p>
<pre><code class="language-json"><span class="token punctuation">{</span>
  <span class="token string-property property">"results"</span><span class="token operator">:</span> <span class="token punctuation">[</span>
    <span class="token punctuation">{</span><span class="token string-property property">"id"</span><span class="token operator">:</span> <span class="token string">"abc123"</span><span class="token punctuation">,</span> <span class="token string-property property">"content"</span><span class="token operator">:</span> <span class="token string">"Task 1"</span><span class="token punctuation">}</span><span class="token punctuation">,</span>
    <span class="token punctuation">{</span><span class="token string-property property">"id"</span><span class="token operator">:</span> <span class="token string">"def456"</span><span class="token punctuation">,</span> <span class="token string-property property">"content"</span><span class="token operator">:</span> <span class="token string">"Task 2"</span><span class="token punctuation">}</span>
  <span class="token punctuation">]</span><span class="token punctuation">,</span>
  <span class="token string-property property">"next_cursor"</span><span class="token operator">:</span> <span class="token string">"eyJwYWdlIjoyLCJsaW1pdCI6NTB9.aGFzaA"</span>
<span class="token punctuation">}</span>
</code></pre>
<p>When <code>next_cursor</code> is <code>null</code>, you&#39;ve reached the end of the results.</p>

<h3 id="first-request">First Request</h3>
<p>To fetch the first page of results, make a request without a cursor parameter:</p>
<pre><code class="language-bash"><span class="token function">curl</span> <span class="token string">"https://api.todoist.com/api/v1/tasks?limit=50"</span> <span class="token punctuation">\</span>
  <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer YOUR_TOKEN"</span>
</code></pre>
<h3 id="subsequent-requests">Subsequent Requests</h3>
<p>To fetch the next page, include the <code>cursor</code> parameter with the value from <code>next_cursor</code>:</p>
<pre><code class="language-bash"><span class="token function">curl</span> <span class="token string">"https://api.todoist.com/api/v1/tasks?cursor=eyJwYWdlIjoyLCJsaW1pdCI6NTB9.aGFzaA&amp;limit=50"</span> <span class="token punctuation">\</span>
  <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer YOUR_TOKEN"</span>
</code></pre>
<p><strong>Important</strong>: Always use the same parameters (filters, sorting, etc.) when using a cursor. Changing parameters between paginated requests may result in unexpected behavior or errors.</p>

<h3 id="parameter-limit">Parameter <code>limit</code></h3>
<p>The <code>limit</code> parameter controls how many objects to return per page.</p>
<ul>
<li><strong>Default</strong>: 50</li>
<li><strong>Maximum</strong>: 200</li>
</ul>
<p>If you specify a limit greater than 200, the API will return a validation error.</p>
<p>Example with custom limit:</p>
<pre><code class="language-bash"><span class="token function">curl</span> <span class="token string">"https://api.todoist.com/api/v1/tasks?limit=100"</span> <span class="token punctuation">\</span>
  <span class="token parameter variable">-H</span> <span class="token string">"Authorization: Bearer YOUR_TOKEN"</span>
</code></pre>
<h3 id="parameter-cursor">Parameter <code>cursor</code></h3>
<p>The <code>cursor</code> parameter is an opaque token returned in the <code>next_cursor</code> field of the previous response.</p>
<p>Cursors are user-specific and parameter-dependent, meaning they can only be used by the same user with the same request parameters (filters, project_id, etc.). Do not attempt to decode, parse, or modify cursors—pass them as-is from the previous response.</p>
<p>See <a href="#best-practices">Best Practices</a> for handling common scenarios.</p>

<ol>
<li><p><strong>Handle concurrent modifications</strong>: Todoist data may change while you&#39;re paginating (you or collaborators adding/removing items). This can cause items to appear twice or be skipped. If consistency is critical, implement deduplication logic in your application.</p>
</li>
<li><p><strong>Don&#39;t store cursors long-term</strong>: Cursors are meant for immediate pagination sessions. Don&#39;t persist them in databases or configuration files.</p>
</li>
<li><p><strong>Process all pages or stop early</strong>: If you need all results, continue fetching pages until <code>next_cursor</code> is <code>null</code>. Stop early if you&#39;ve found what you need.</p>
</li>
</ol>

<h3 id="invalid-cursor">Invalid Cursor</h3>
<p>If you provide a malformed or tampered cursor:</p>
<pre><code class="language-json"><span class="token punctuation">{</span>
  <span class="token string-property property">"error"</span><span class="token operator">:</span> <span class="token string">"Invalid argument value"</span><span class="token punctuation">,</span>
  <span class="token string-property property">"error_code"</span><span class="token operator">:</span> <span class="token number">20</span><span class="token punctuation">,</span>
  <span class="token string-property property">"error_extra"</span><span class="token operator">:</span> <span class="token punctuation">{</span>
    <span class="token string-property property">"argument"</span><span class="token operator">:</span> <span class="token string">"cursor"</span><span class="token punctuation">,</span>
  <span class="token punctuation">}</span><span class="token punctuation">,</span>
  <span class="token string-property property">"error_tag"</span><span class="token operator">:</span> <span class="token string">"INVALID_ARGUMENT_VALUE"</span><span class="token punctuation">,</span>
  <span class="token string-property property">"http_code"</span><span class="token operator">:</span> <span class="token number">400</span>
<span class="token punctuation">}</span>
</code></pre>
<p><strong>Solution</strong>: Use the cursor exactly as returned from the previous response, or restart pagination from the beginning without a cursor parameter.</p>
<h3 id="invalid-limit-value">Invalid Limit Value</h3>
<p>If you provide a limit greater than 200:</p>
<pre><code class="language-json"><span class="token punctuation">{</span>
  <span class="token string-property property">"error"</span><span class="token operator">:</span> <span class="token string">"Invalid argument value"</span><span class="token punctuation">,</span>
  <span class="token string-property property">"error_code"</span><span class="token operator">:</span> <span class="token number">20</span><span class="token punctuation">,</span>
  <span class="token string-property property">"error_extra"</span><span class="token operator">:</span> <span class="token punctuation">{</span>
    <span class="token string-property property">"argument"</span><span class="token operator">:</span> <span class="token string">"limit"</span><span class="token punctuation">,</span>
    <span class="token string-property property">"expected"</span><span class="token operator">:</span> <span class="token string">"Input should be less than or equal to 200"</span><span class="token punctuation">,</span>
  <span class="token punctuation">}</span><span class="token punctuation">,</span>
  <span class="token string-property property">"error_tag"</span><span class="token operator">:</span> <span class="token string">"INVALID_ARGUMENT_VALUE"</span><span class="token punctuation">,</span>
  <span class="token string-property property">"http_code"</span><span class="token operator">:</span> <span class="token number">400</span>
<span class="token punctuation">}</span>
</code></pre>
<p><strong>Solution</strong>: Use a limit value of 200 or less.</p>

<p>Here&#39;s a Python example that fetches all tasks using pagination:</p>
<pre><code class="language-python"><span class="token keyword">import</span> requests

token <span class="token operator">=</span> <span class="token string">"YOUR_TOKEN"</span>
url <span class="token operator">=</span> <span class="token string">"https://api.todoist.com/api/v1/tasks"</span>
headers <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token string">"Authorization"</span><span class="token punctuation">:</span> <span class="token string-interpolation"><span class="token string">f"Bearer </span><span class="token interpolation"><span class="token punctuation">{</span>token<span class="token punctuation">}</span></span><span class="token string">"</span></span><span class="token punctuation">}</span>

all_tasks <span class="token operator">=</span> <span class="token punctuation">[</span><span class="token punctuation">]</span>
cursor <span class="token operator">=</span> <span class="token boolean">None</span>

<span class="token keyword">while</span> <span class="token boolean">True</span><span class="token punctuation">:</span>
    params <span class="token operator">=</span> <span class="token punctuation">{</span><span class="token string">"limit"</span><span class="token punctuation">:</span> <span class="token number">100</span><span class="token punctuation">}</span>
    <span class="token keyword">if</span> cursor<span class="token punctuation">:</span>
        params<span class="token punctuation">[</span><span class="token string">"cursor"</span><span class="token punctuation">]</span> <span class="token operator">=</span> cursor

    response <span class="token operator">=</span> requests<span class="token punctuation">.</span>get<span class="token punctuation">(</span>url<span class="token punctuation">,</span> headers<span class="token operator">=</span>headers<span class="token punctuation">,</span> params<span class="token operator">=</span>params<span class="token punctuation">)</span>
    response<span class="token punctuation">.</span>raise_for_status<span class="token punctuation">(</span><span class="token punctuation">)</span>
    data <span class="token operator">=</span> response<span class="token punctuation">.</span>json<span class="token punctuation">(</span><span class="token punctuation">)</span>

    all_tasks<span class="token punctuation">.</span>extend<span class="token punctuation">(</span>data<span class="token punctuation">[</span><span class="token string">"results"</span><span class="token punctuation">]</span><span class="token punctuation">)</span>

    cursor <span class="token operator">=</span> data<span class="token punctuation">.</span>get<span class="token punctuation">(</span><span class="token string">"next_cursor"</span><span class="token punctuation">)</span>
    <span class="token keyword">if</span> <span class="token keyword">not</span> cursor<span class="token punctuation">:</span>
        <span class="token keyword">break</span>

<span class="token keyword">print</span><span class="token punctuation">(</span><span class="token string-interpolation"><span class="token string">f"Fetched </span><span class="token interpolation"><span class="token punctuation">{</span><span class="token builtin">len</span><span class="token punctuation">(</span>all_tasks<span class="token punctuation">)</span><span class="token punctuation">}</span></span><span class="token string"> tasks"</span></span><span class="token punctuation">)</span>
</code></pre>

<p>The <code>/api/v1/activities</code> endpoint uses a different pagination mechanism than the cursor-based pagination described in this guide. See the <a href="#tag/Activity">Activities documentation</a> for details on how to paginate activity log results.</p>

<h3 id="payload-size">Payload Size</h3>
<p>There is a 1MiB HTTP request body limit on POST requests.</p>
<p>The maximum payload size for an <a href="#uploads">attachment upload</a> is dependent on the current user plan.
This value is indicated by the <code>upload_limit_mb</code> property of the <a href="#user-plan-limits">user plan limits</a> object.</p>
<h3 id="header-size">Header Size</h3>
<p>Total size of HTTP headers cannot exceed 65 KiB.</p>
<h3 id="processing-timeouts">Processing Timeouts</h3>
<p>There are processing timeouts associated with each endpoint, and these vary
depending on the type of action being performed.</p>
<table>
<thead>
<tr>
<th>Type</th>
<th>Limit</th>
</tr>
</thead>
<tbody><tr>
<td>Uploads</td>
<td>5 minutes</td>
</tr>
<tr>
<td>Standard Request</td>
<td>15 seconds</td>
</tr>
</tbody></table>
<h3 id="rate-limiting">Rate Limiting</h3>
<p>Limits are applied differently for full and partial syncs. You should ideally only make a full sync on your initial request and then subsequently perform incremental syncs as this is faster and more efficient.</p>
<p>See the sync section for further information on <a href="#read-resources">incremental sync</a>.</p>
<p>For each user, you can make a maximum of 1000 partial sync requests within a 15 minute period.</p>
<p>For each user, you can make a maximum of 100 full sync requests within a 15 minute period.</p>
<p>You can reduce the number of requests you make by batching up to 100 commands in each request and it will still count as one.
See the <a href="#batching-commands">Batching Commands</a> section for further information.</p>
<h3 id="maximum-sync-commands">Maximum Sync Commands</h3>
<p>The maximum number of commands is 100 per request. This restriction is applied to prevent
timeouts and other problems when dealing with large requests.</p>

<p>Our applications for <a href="https://play.google.com/store/apps/details?id=com.todoist">Android</a> and <a href="https://apps.apple.com/us/app/todoist-to-do-list-calendar/id572688855">iOS</a> support custom URL schemes for launching to specific views and initiating some common actions.</p>

<p>The following schemes are available to open a specific view:</p>
<table>
<thead>
<tr>
<th>Scheme</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>todoist://</td>
<td>Opens Todoist to the user&#39;s default view.</td>
</tr>
<tr>
<td>todoist://today</td>
<td>Opens the today view.</td>
</tr>
<tr>
<td>todoist://upcoming</td>
<td>Opens the Upcoming view.</td>
</tr>
<tr>
<td>todoist://profile</td>
<td>Opens the profile view.</td>
</tr>
<tr>
<td>todoist://inbox</td>
<td>Opens the inbox view.</td>
</tr>
<tr>
<td>todoist://teaminbox</td>
<td>Opens the team inbox view. If the user doesn&#39;t have a business account it will show an alert and redirect automatically to the inbox view.</td>
</tr>
<tr>
<td>todoist://notifications</td>
<td>Opens notifications view.</td>
</tr>
</tbody></table>
<h3 id="tasks">Tasks</h3>
<blockquote>
<p>Example of adding a task:</p>
</blockquote>
<pre><code class="language-text">todoist://addtask?content=mytask&amp;date=tomorrow&amp;priority=4
</code></pre>
<blockquote>
<p>Here&#39;s an example of a content value:</p>
</blockquote>
<pre><code class="language-text">Create document about URL Schemes!
</code></pre>
<blockquote>
<p>And how it should be supplied using Percent-encoding:</p>
</blockquote>
<pre><code class="language-text">Create&amp;20document%20about%20URL%20Schemes%21
</code></pre>
<blockquote>
<p>Here&#39;s an example of a date value:</p>
</blockquote>
<pre><code class="language-text">Tomorrow @ 14:00
</code></pre>
<blockquote>
<p>And how it should be supplied using Percent-encoding:</p>
</blockquote>
<pre><code class="language-text">Tomorrow%20@%2014:00
</code></pre>
<p>The following schemes are available for tasks:</p>
<table>
<thead>
<tr>
<th>Scheme</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>todoist://task?id={id}</td>
<td>Opens a task by ID.</td>
</tr>
<tr>
<td>todoist://addtask</td>
<td>Opens the add task view to add a new task to Todoist.</td>
</tr>
</tbody></table>
<p>The <code>todoist://addtask</code> scheme accepts the following optional values:</p>
<table>
<thead>
<tr>
<th>Value</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>content <em>URL encoding</em></td>
<td>The content of the task, which should be a string that is in <code>Percent-encoding</code> (also known as URL encoding).</td>
</tr>
<tr>
<td>date <em>URL encoding</em></td>
<td>The due date of the task, which should be a string that is in <code>Percent-encoding</code> (also known as URL encoding). Look at our reference to see <a href="https://www.todoist.com/help/articles/introduction-to-due-dates-and-due-times-q7VobO">which formats are supported</a>.</td>
</tr>
<tr>
<td>priority <em>Integer</em></td>
<td>The priority of the task (a number between <code>1</code> and <code>4</code>, <code>4</code> for very urgent and <code>1</code> for natural). <br><strong>Note</strong>: Keep in mind that <code>very urgent</code> is the priority 1 on clients. So, <code>p1</code> will return <code>4</code> in the API.</td>
</tr>
</tbody></table>
<p>This URL scheme will not automatically submit the task to Todoist, it will just open and pre-fill the add task view. If no values are passed, the add task view will just be opened.</p>

<p>The following schemes are available for tasks:</p>
<table>
<thead>
<tr>
<th>Scheme</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>todoist://projects</td>
<td>Opens the projects view (shows all projects).</td>
</tr>
<tr>
<td>todoist://project?id={id}</td>
<td>Opens a specific project by ID.</td>
</tr>
</tbody></table>
<blockquote>
<p>Example of opening a specific project:</p>
</blockquote>
<pre><code class="language-text">todoist://project?id=128501470
</code></pre>
<p>The <code>todoist://project</code> scheme accepts the following required value:</p>
<table>
<thead>
<tr>
<th>Value</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>Integer</em></td>
<td>The ID of the project to view. If the ID doesn&#39;t exist, you don&#39;t have access to the project, or the value is empty, an alert will be showed and the user will be redirected to the projects view.</td>
</tr>
</tbody></table>
<h3 id="labels">Labels</h3>
<p>The following schemes are available for labels:</p>
<table>
<thead>
<tr>
<th>Scheme</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>todoist://labels</td>
<td>Opens the labels view (shows all labels)</td>
</tr>
<tr>
<td>todoist://label?name={name}</td>
<td>Opens a specific label by name.</td>
</tr>
</tbody></table>
<blockquote>
<p>Example of opening a specific label:</p>
</blockquote>
<pre><code class="language-text">todoist://label?name=Urgent
</code></pre>
<p>The <code>todoist://label</code> scheme accepts the following required value:</p>
<table>
<thead>
<tr>
<th>Value</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>name <em>String</em></td>
<td>The name of the label to view. If the label doesn&#39;t exist, you don&#39;t have access to the label, or the value is empty, an alert will be shown.</td>
</tr>
</tbody></table>
<h3 id="filters">Filters</h3>
<p>The following schemes are available for filters:</p>
<table>
<thead>
<tr>
<th>Scheme</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>todoist://filters</td>
<td>Opens the filters view (shows all filters)</td>
</tr>
<tr>
<td>todoist://filter?id={id}</td>
<td>Opens a specific filter by ID.</td>
</tr>
</tbody></table>
<blockquote>
<p>Example of opening a specific filter:</p>
</blockquote>
<pre><code class="language-text">todoist://filter?id=9
</code></pre>
<p>The <code>todoist://filter</code> scheme accepts the following required value:</p>
<table>
<thead>
<tr>
<th>Value</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>Integer</em></td>
<td>The ID of the filter to view. If the ID doesn&#39;t exist, you don’t have access to the filter, or the value is empty, an alert will be showed and the user will be redirected to the filters view.</td>
</tr>
</tbody></table>
<h3 id="search">Search</h3>
<p>The following scheme is available for searching (Android only):</p>
<table>
<thead>
<tr>
<th>Scheme</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>todoist://search?query={query}</td>
<td>Used to search in the Todoist application.</td>
</tr>
</tbody></table>
<blockquote>
<p>Example of searching for &quot;Test &amp; Today&quot;:</p>
</blockquote>
<pre><code class="language-text">todoist://search?query=Test%20%26%20Today
</code></pre>
<p>The <code>todoist://search</code> scheme accepts the following required value:</p>
<table>
<thead>
<tr>
<th>Value</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>query <em>URL encoding</em></td>
<td>The query to search in the Todoist application, which should be a string that is in <code>Percent-encoding</code> (also known as URL encoding).</td>
</tr>
</tbody></table>

<p>Our <a href="https://todoist.com/downloads">Desktop</a> applications support custom URL schemes for launching to specific views and initiating some common actions. This can be useful for integrating Todoist with other applications or services, as browsers and other applications can open these URLs to interact with Todoist.
As an example, you could create a link in your application that opens a specific project in Todoist, or a link that adds a task to Todoist.</p>
<h3 id="views">Views</h3>
<p>The following schemes are available to open a specific view:</p>
<table>
<thead>
<tr>
<th>Scheme</th>
<th>Description</th>
<th>minimum version requirement</th>
</tr>
</thead>
<tbody><tr>
<td>todoist://</td>
<td>Opens Todoist.</td>
<td>9.2.0</td>
</tr>
<tr>
<td>todoist://inbox</td>
<td>Opens the inbox view.</td>
<td>9.2.0</td>
</tr>
<tr>
<td>todoist://today</td>
<td>Opens the today view.</td>
<td>9.2.0</td>
</tr>
<tr>
<td>todoist://upcoming</td>
<td>Opens the Upcoming view.</td>
<td>9.2.0</td>
</tr>
<tr>
<td>todoist://project?id={id}</td>
<td>Opens the project detail view for a given project ID.</td>
<td>9.2.0</td>
</tr>
<tr>
<td>todoist://task?id={id}</td>
<td>Opens the task detail view for a given task ID.</td>
<td>9.2.0</td>
</tr>
<tr>
<td>todoist://openquickadd?content={content}&amp;description={description}</td>
<td>Opens the global quick add, optionally refilled.</td>
<td>9.2.0</td>
</tr>
<tr>
<td>todoist://notifications</td>
<td>Opens the notifications view.</td>
<td>9.10.0</td>
</tr>
<tr>
<td>todoist://filters-labels</td>
<td>Opens the filters &amp; labels view.</td>
<td>9.10.0</td>
</tr>
<tr>
<td>todoist://filter?id={id}</td>
<td>Opens the filter view for a given filter ID.</td>
<td>9.10.0</td>
</tr>
<tr>
<td>todoist://label?id={id}</td>
<td>Opens the label view for a given label ID.</td>
<td>9.10.0</td>
</tr>
<tr>
<td>todoist://search?query={query}</td>
<td>Opens the search view for the specified query.</td>
<td>9.10.0</td>
</tr>
<tr>
<td>todoist://projects</td>
<td>Opens my projects view.</td>
<td>9.10.0</td>
</tr>
<tr>
<td>todoist://projects?workspaceId={id}</td>
<td>Opens the projects view for the given workspace ID.</td>
<td>9.10.0</td>
</tr>
<tr>
<td>todoist://templates</td>
<td>Opens the templates view.</td>
<td>9.10.0</td>
</tr>
<tr>
<td>todoist://templates?id={id}</td>
<td>Opens the template view for the given template ID.</td>
<td>9.10.0</td>
</tr>
</tbody></table>
<h3 id="tasks">Tasks</h3>
<blockquote>
<p>Example of adding a task:</p>
</blockquote>
<p><em>Note that this will not add the task but open the Global Quick Add refilled with given values.</em></p>
<pre><code class="language-text">todoist://openquickadd?content=mytask&amp;description=%20is%20a%20description
</code></pre>
<p>The following schemes are available for tasks:</p>
<table>
<thead>
<tr>
<th>Scheme</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>todoist://task?id={id}</td>
<td>Opens a task by ID.</td>
</tr>
<tr>
<td>todoist://openquickadd</td>
<td>Opens the global quick add to add a new task to Todoist.</td>
</tr>
</tbody></table>
<p>The <code>todoist://openquickadd</code> scheme accepts the following optional values:</p>
<table>
<thead>
<tr>
<th>Value</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>content <em>URL encoding</em></td>
<td>The content of the task, which should be a string that is in <code>Percent-encoding</code> (also known as URL encoding).</td>
</tr>
<tr>
<td>description <em>URL encoding</em></td>
<td>The content of the task, which should be a string that is in <code>Percent-encoding</code> (also known as URL encoding).</td>
</tr>
</tbody></table>
<p>This URL scheme will not automatically submit the task to Todoist, it will just open and pre-fill the global quick add panel. If no values are passed, the global quick add will just be open.</p>
<h3 id="projects">Projects</h3>
<p>The following schemes are available for projects:</p>
<table>
<thead>
<tr>
<th>Scheme</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>todoist://project?id={id}</td>
<td>Opens a specific project by ID.</td>
</tr>
</tbody></table>
<blockquote>
<p>Example of opening a specific project:</p>
</blockquote>
<pre><code class="language-text">todoist://project?id=128501470
</code></pre>
<p>The <code>todoist://project</code> scheme accepts the following required value:</p>
<table>
<thead>
<tr>
<th>Value</th>
<th>Description</th>
</tr>
</thead>
<tbody><tr>
<td>id <em>Integer</em></td>
<td>The ID of the project to view. If the ID doesn&#39;t exist it will just open Todoist. If you don&#39;t have access to the project, or the project does not exist, an error message will be shown to the user.</td>
</tr>
</tbody></table>

<p>The Todoist API v1 is a new API that unifies the Sync API v9 and the REST API
v2. This section shows what was changed in the new version in one single
place to ease the migration for current apps and integrations.</p>
<p>The documentation for the <a href="https://developer.todoist.com/sync/v9">Sync API v9</a>
and <a href="https://developer.todoist.com/rest/v2">REST API v2</a> are still available
for reference.</p>

<p>Up until now, Todoist&#39;s endpoints were case-insensitive. The Todoist API v1
will make endpoints default to lowercase (mostly snake_case) and reject mixed casing.</p>
<p>As an example:</p>
<p><a href="https://api.todoist.com/API/v9/Sync">https://api.todoist.com/API/v9/Sync</a></p>
<p>would before be accepted in the same way as:</p>
<p><a href="https://api.todoist.com/api/v9/sync">https://api.todoist.com/api/v9/sync</a></p>
<p>but now, the former will return 404.</p>
<p>Please confirm you&#39;re only issuing requests to lowercase endpoints.</p>

<p>After Todoist API v1, we will only focus on <code>api.todoist.com</code> as the subdomain.</p>
<p>If you&#39;re using any other subdomain, please migrate your API requests to <code>api.todoist.com</code> as documented.</p>

<p>Since 2023, our objects returned <code>v2_*_id</code> attributes. That &quot;v2 id&quot; has now become the main <code>id</code>.</p>
<p>IDs have been opaque strings almost everywhere since the release of Sync API v9,
but were still mostly numbers in that version. This version officially makes
them non-number opaque strings, changing the old IDs.</p>
<p>The <code>v2_*_id</code> attribute is still available on Sync API v9, but was removed on the new version.
We suggest relying on them for migrating stored or cached data before bumping the major version.</p>
<p>You can also rely on the following endpoint to translate between both ID versions:
<a href="#tag/Ids/operation/id_mappings_api_v1_id_mappings__obj_name___obj_ids__get"><code>/api/v1/ids_mapping/&lt;object&gt;/&lt;id&gt;[,&lt;id&gt;]</code></a>.
It supports up to 100 IDs (of the same object) at a time.</p>
<p>Old IDs will NOT be accepted in this new API version for the following objects:</p>
<ul>
<li>notes / comments</li>
<li>items / tasks</li>
<li>projects</li>
<li>sections</li>
<li>notifications / reminders</li>
<li>notifications_locations / location_reminder</li>
</ul>
<p>Trying to use old IDs will result in an error.</p>

<p>The previous task object included a <code>url</code> property:</p>
<pre><code><span class="token string">"url"</span><span class="token punctuation">:</span> <span class="token string">"https://todoist.com/showTask?id=&lt;v1_id>>"</span>
</code></pre>
<p>This has been removed.  See below for information regarding the format for task URLs going forward.</p>
<p>Valid Task URLs are formatted as follows:</p>
<pre><code>https<span class="token punctuation">:</span><span class="token operator">/</span><span class="token operator">/</span>app<span class="token punctuation">.</span>todoist<span class="token punctuation">.</span>com<span class="token operator">/</span>app<span class="token operator">/</span>task<span class="token operator">/</span><span class="token operator">&lt;</span>v2_id<span class="token operator">></span>
</code></pre>

<p>This version adds pagination to many endpoints.</p>
<p>The following endpoints are now paginated:</p>
<ul>
<li><code>/api/v1/tasks</code></li>
<li><code>/api/v1/tasks/filter</code></li>
<li><code>/api/v1/labels</code></li>
<li><code>/api/v1/labels/shared</code></li>
<li><code>/api/v1/comments</code></li>
<li><code>/api/v1/sections</code></li>
<li><code>/api/v1/projects</code></li>
<li><code>/api/v1/projects/archived</code></li>
<li><code>/api/v1/projects/&lt;project_id&gt;/collaborators</code></li>
<li><code>/api/v1/activities</code></li>
</ul>
<p>They all use cursor-based pagination. See the <a href="#tag/Pagination">Pagination guide</a> for complete details.</p>

<p>All endpoints related to <code>/tasks</code>, <code>/comments</code>, <code>/sections</code>, <code>/projects</code>, and
<code>/labels</code> were returning <code>plain/text</code> error responses before the Todoist API v1.
With the unification of the APIs, we have now unified the error response to return
<code>application/json</code> on these endpoints.</p>
<p>Instead of:</p>
<pre><code>Content<span class="token operator">-</span>type<span class="token punctuation">:</span> plain<span class="token operator">/</span>text
Task not found
</code></pre>
<p>It will return:</p>
<pre><code class="language-json">Content<span class="token operator">-</span>type<span class="token operator">:</span> application<span class="token operator">/</span>json
<span class="token punctuation">{</span>
  <span class="token string-property property">'error'</span><span class="token operator">:</span> <span class="token string">'Task not found'</span><span class="token punctuation">,</span>
  <span class="token string-property property">'error_code'</span><span class="token operator">:</span> <span class="token number">478</span><span class="token punctuation">,</span>
  <span class="token string-property property">'error_extra'</span><span class="token operator">:</span> <span class="token punctuation">{</span><span class="token string-property property">'event_id'</span><span class="token operator">:</span> <span class="token string">'&lt;hash>'</span><span class="token punctuation">,</span> <span class="token string-property property">'retry_after'</span><span class="token operator">:</span> <span class="token number">3</span><span class="token punctuation">}</span><span class="token punctuation">,</span>
  <span class="token string-property property">'error_tag'</span><span class="token operator">:</span> <span class="token string">'NOT_FOUND'</span><span class="token punctuation">,</span>
  <span class="token string-property property">'http_code'</span><span class="token operator">:</span> <span class="token number">404</span>
<span class="token punctuation">}</span>
</code></pre>
<p>This is the same format used in the previous Sync API, which is now the default for the new Todoist API.</p>

<p>The API kept the old names of objects for a long time to avoid breaking
compatibility, but the unification of APIs was the perfect time to unformize.</p>
<p>The Todoist API v1 renames objects to match what users currently see in the app:</p>
<table>
<thead>
<tr>
<th>Sync v9 / REST v2</th>
<th>Todoist API v1</th>
</tr>
</thead>
<tbody><tr>
<td>items</td>
<td>tasks</td>
</tr>
<tr>
<td>notes</td>
<td>comments</td>
</tr>
<tr>
<td>notifications</td>
<td>reminders</td>
</tr>
<tr>
<td>notifications_locations</td>
<td>location_reminders</td>
</tr>
</tbody></table>
<p>The nomenclature listed on the left in the table above, should be renamed to the associated term to the right, unless a documented exception exists.</p>
<p>The only exceptions for renaming are the <code>/sync</code> and <code>/activities</code> endpoints. These are currently scheduled for bigger
architectural refactoring in the near future, so we will retain the the old naming conventions for now.</p>

<p>With the unification of both APIs, we took the chance to unify concepts and improve some URLs to new standards. These are the endpoint signature changes:</p>
<table>
<thead>
<tr>
<th>Sync v9 / REST v2</th>
<th>Todoist API v1</th>
</tr>
</thead>
<tbody><tr>
<td><code>/api/v9/update_notification_setting</code></td>
<td>PUT <code>/api/v1/notification_setting</code></td>
</tr>
<tr>
<td><code>/api/v9/uploads/add</code></td>
<td>POST <code>/api/v1/uploads</code></td>
</tr>
<tr>
<td><code>/api/v9/uploads/get</code></td>
<td>GET <code>/api/v1/uploads</code></td>
</tr>
<tr>
<td><code>/api/v9/uploads/delete</code></td>
<td>DELETE <code>/api/v1/uploads</code></td>
</tr>
<tr>
<td><code>/api/v9/backups/get</code></td>
<td>GET <code>/api/v1/backups</code></td>
</tr>
<tr>
<td><code>/api/v9/access_tokens/revoke</code></td>
<td>DELETE <code>/api/v1/access_tokens</code></td>
</tr>
<tr>
<td><code>/api/access_tokens/revoke</code></td>
<td>DELETE <code>/api/v1/access_tokens</code></td>
</tr>
<tr>
<td><code>/api/access_tokens/migrate_personal_token</code></td>
<td>POST <code>/api/v1/access_tokens/migrate_personal_token</code></td>
</tr>
<tr>
<td><code>/api/v9/access_tokens/migrate_personal_token</code></td>
<td>POST <code>/api/v1/access_tokens/migrate_personal_token</code></td>
</tr>
<tr>
<td><code>/api/v9/archive/sections</code></td>
<td>GET <code>/api/v1/sections/archived</code></td>
</tr>
<tr>
<td><code>/api/v9/quick/add</code></td>
<td>POST <code>/api/v1/tasks/quick</code></td>
</tr>
<tr>
<td><code>/api/v9/emails/get_or_create</code></td>
<td>PUT <code>/api/v1/emails</code></td>
</tr>
<tr>
<td><code>/api/v9/emails/disable</code></td>
<td>DELETE <code>/api/v1/emails</code></td>
</tr>
<tr>
<td><code>/api/v9/get_productivity_stats</code></td>
<td>GET <code>/api/v1/tasks/completed/stats</code></td>
</tr>
<tr>
<td><code>/api/v9/completed/get_stats</code></td>
<td>GET <code>/api/v1/tasks/completed/stats</code></td>
</tr>
<tr>
<td><code>/api/v9/completed/get_all</code></td>
<td>GET <code>/api/v1/tasks/completed</code></td>
</tr>
<tr>
<td><code>/api/v9/projects/get_archived</code></td>
<td>GET <code>/api/v1/projects/archived</code></td>
</tr>
<tr>
<td><code>/api/v9/projects/join</code></td>
<td>POST <code>/api/v1/projects/&lt;project_id&gt;/join</code></td>
</tr>
<tr>
<td><code>/api/v9/workspaces/projects/active</code></td>
<td>GET <code>/api/v1/workspaces/&lt;workspace_id&gt;/projects/active</code></td>
</tr>
<tr>
<td><code>/api/v9/workspaces/projects/archived</code></td>
<td>GET <code>/api/v1/workspaces/&lt;workspace_id&gt;/projects/archived</code></td>
</tr>
<tr>
<td><code>/api/v9/workspaces/update_logo</code></td>
<td>POST <code>/api/v1/workspaces/logo</code></td>
</tr>
<tr>
<td><code>/api/v9/workspaces/invitations/accept</code></td>
<td>PUT <code>/api/v1/workspaces/invitations/&lt;invitation_code&gt;/accept</code></td>
</tr>
<tr>
<td><code>/api/v9/workspaces/invitations/reject</code></td>
<td>PUT <code>/api/v1/workspaces/invitations/&lt;invitation_code&gt;/reject</code></td>
</tr>
<tr>
<td><code>/api/v9/workspaces/joinable_workspaces</code></td>
<td>GET <code>/api/v1/workspaces/joinable</code></td>
</tr>
<tr>
<td><code>/api/v9/projects/get_data</code></td>
<td>GET <code>/api/v1/projects/&lt;project_id&gt;/full</code></td>
</tr>
<tr>
<td><code>/api/v9/templates/import_into_project</code></td>
<td>POST <code>/api/v1/templates/import_into_project_from_file</code></td>
</tr>
<tr>
<td><code>/api/v9/templates/export_as_file</code></td>
<td>GET <code>/api/v1/templates/file</code></td>
</tr>
<tr>
<td><code>/api/v9/templates/export_as_url</code></td>
<td>GET <code>/api/v1/templates/url</code></td>
</tr>
<tr>
<td><code>/api/v9/activity/get</code></td>
<td>GET <code>/api/v1/activities</code></td>
</tr>
<tr>
<td><code>/api/v9/tasks/archived/by_due_date</code></td>
<td>GET <code>/api/v1/tasks/completed/by_due_date</code></td>
</tr>
<tr>
<td><code>/api/v9/tasks/completed/by_completion_date</code></td>
<td>GET <code>/api/v1/tasks/completed/by_completion_date</code></td>
</tr>
</tbody></table>

<p>There are some endpoints that were previously available in the Sync or REST
APIs, but were removed from the Todoist API v1. Below is a list of them and
possible candidates for replacement:</p>
<table>
<thead>
<tr>
<th>Sync v9 / REST v2</th>
<th>New endpoint taking its place</th>
</tr>
</thead>
<tbody><tr>
<td><code>/sync/v9/archive/items_many</code></td>
<td><code>/api/v1/tasks/completed/by_completion_date</code></td>
</tr>
<tr>
<td><code>/sync/v9/archive/items</code></td>
<td><code>/api/v1/tasks/completed/by_completion_date</code></td>
</tr>
<tr>
<td><code>/sync/v9/completed/get_all</code></td>
<td><code>/api/v1/tasks/completed/by_completion_date</code></td>
</tr>
<tr>
<td><code>/sync/v9/projects/get</code></td>
<td><code>/api/v1/projects</code>, <code>/api/v1/comment</code></td>
</tr>
<tr>
<td><code>/sync/v9/items/get</code></td>
<td><code>/api/v1/tasks</code>, <code>/api/v1/comments</code>, <code>/api/v1/projects</code>, <code>/api/v1/sections</code></td>
</tr>
<tr>
<td><code>/sync/v9/projects/get_data</code></td>
<td><code>/api/v1/tasks</code>, <code>/api/v1/comments</code>, <code>/api/v1/projects</code>, <code>/api/v1/sections</code></td>
</tr>
</tbody></table>

<ul>
<li>This endpoint is one of the exceptions for <a href="#tag/Migrating-from-v9/Object-renames">object
renames</a>, with legacy naming still in use</li>
<li><code>day_orders_timestamp</code> attribute was removed from the response on the <code>/sync</code>
endpoint</li>
<li>A new <code>full_sync_date_utc</code> attribute is included during initial sync, with the
time when that sync data was generated. For big accounts, the data may be
returned with some delay; doing an <a href="#tag/Sync/Overview/Incremental-sync">incremental
sync</a> afterwards is required to get
up-to-date data.</li>
</ul>

<ul>
<li><code>collapsed</code> attribute was renamed to <code>is_collapsed</code></li>
</ul>

<ul>
<li><code>is_biz_admin</code> attribute was removed</li>
</ul>

<ul>
<li><code>uncompleted_tasks_count</code> and <code>total_tasks_count</code> were removed from <a href="#tag/Workspace/operation/active_projects_api_v1_workspaces__workspace_id__projects_active_get">Workspace Projects</a></li>
</ul>

<ul>
<li>The <code>comment_count</code> attribute was removed from the response: this applies to all <code>/tasks*</code> endpoints.</li>
<li>The <code>filter</code> and <code>lang</code> parameters were removed: A new dedicated endpoint has been created specifically for filtering tasks: <code>/api/v1/tasks/filter</code>. This new endpoint allows for the same filtering capabilities but with a more specialized API surface.</li>
</ul>

<ul>
<li>The <code>comment_count</code> attribute was removed from the response. This applies to all <code>/projects*</code> endpoints.</li>
</ul>

<p>Sections used a slightly different response format in the Sync and REST APIs.
The Todoist API v1 uses the format previously used by the Sync API everywhere.</p>

<p>Comments a used slightly different response format in the Sync and REST APIs.
The Todoist API v1 uses the format previously used by the Sync API everywhere.</p>

<p>There are no changes specific to webhooks, but they will inherit all the other formatting and renaming changes outlined above. Developers are expected <a href="https://developer.todoist.com/appconsole.html">to change the version of the webhook for their integration</a> and start accepting the new formatting once the integration is ready to handle it.</p>

