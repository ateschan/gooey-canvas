<script lang="ts">
  import { Router, Link, Route } from "svelte-routing";
  import Greet from "./lib/Greet.svelte";
  import Home from "./lib/Home.svelte";
  import CourseDetail from "./lib/CourseDetail.svelte";

  //Added Routing with svelte-routing
</script>

<Router>
  <main class="container app">
    <div>
      <Route path="/lib/Home"><Home /></Route>
      <Route path="/lib/CourseDetail"><CourseDetail /></Route>
      <Route path="/"><Greet /></Route>
    </div>
  </main>
</Router>
