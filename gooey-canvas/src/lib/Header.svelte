<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { title } from "./stores";

  //Header component will hold the users name at Home page, and course name in
</script>

<main class="header-section">
  <h2 class="header">{$title}</h2>
</main>
