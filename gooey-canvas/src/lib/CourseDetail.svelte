<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Header from "./Header.svelte";
  import { navigate } from "svelte-routing";

  //This compoment is the page where course assingments, grades and info
  //is layed out, including graphs and data.

  //TODO: make back button look nice
  //TODO: invoke get_user_course_assingmemts
</script>

<main>
  <div class="header">
    <button
      on:click={() => {
        navigate("/lib/Home", { replace: true });
      }}>BACK</button
    >
    <Header />
  </div>
  <p>Add stuff here</p>
</main>
