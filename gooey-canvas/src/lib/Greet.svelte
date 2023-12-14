<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let key = "";
  let greetMsg = "";

  let courseData = ""; //Holds API Data
  let classesText = ""; //Shows "Classes:" when greet executes

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { key });
    courseData = await invoke("get_user_courses");

    classesText = "Classes:";
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input
      id="greet-input"
      placeholder="Enter an API Key..."
      bind:value={key}
    />
    <button type="submit">Submit</button>
  </form>
  <p>{greetMsg}</p>
  <h3>{classesText}</h3>

  <!--TODO add button support to show something when clicked-->
  <!--Attempted creating a toggle function to handle this, got scared and went home-->
  <!--Errors are showing for object item and its keys, not sure why-->
  <ol>
    {#each courseData as item, index (item.course_id)}
      <li>
        <button>{item.course_name}</button>
      </li>
    {/each}
  </ol>
</div>
