<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Header from "./Header.svelte";
  import { navigate } from "svelte-routing";
  import { title } from "./stores";

  let courseData: Object[] = [];

  async function loadData() {
    courseData = await invoke("get_user_courses");
    return courseData;
  }

  //Home component is the users home page after submiting api key
  //it shows users enrolled courses and will navigate to said course

  //TODO: loadData() is called everytime component mounts,
  //find a way to call loadData only once?
</script>

<Header />
<div>
  {#await loadData() then courses}
    <break></break>
    <div>
      {#each courseData as course, index}
        <div>
          <!-- onclick handler sets $title to course name, then navigates to courseDetail -->
          <button
            class="course-button"
            on:click={() => {
              $title = course.course_name;
              navigate("/lib/CourseDetail", { replace: true });
            }}>{course.course_name}</button
          >
        </div>
      {/each}
    </div>
  {/await}
</div>
