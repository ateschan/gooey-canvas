<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let key = "";
  let greetMsg = "";

  let courseData: Object[] = []; //Holds API Data
  let classesText = ""; //Shows "Classes:" when greet executes

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { key });
    courseData = await invoke("get_user_courses");

    classesText = "Classes:";

    //for each assigns the isShowing key value pair to each courseData object
    courseData.forEach((course) => {
      course.isShowing = false;
    });
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

  <!--Callback function in button is used to open and close each course tab-->
  <!--TODO: add styles and maybe background in if statement-->
  <!--Errors are showing for object item and its keys, not sure why-->
  <ol>
    {#each courseData as course, index}
      <li>
        <button
          on:click={() => {
            courseData[index].isShowing = !courseData[index].isShowing;
          }}>{course.course_name}</button
        >
      </li>
      {#if course.isShowing}
        <h3>Grade: {course.grades.current_grade}</h3>
      {/if}
    {/each}
  </ol>
</div>
