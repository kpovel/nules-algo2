<script lang="ts">
  import "../app.css";
  import { page } from "$app/stores";
  $: currentPath = $page.url.pathname;

  const navLinks = [
    { path: "/", title: "Sort results" },
    { path: "/about-me", title: "About me" }
  ];
</script>

<header class="bg-sky-300">
  <nav class="flex h-16 items-center justify-center gap-5">
    {#each navLinks as item}
      <a
        href={item.path}
        class="rounded-lg border px-4 py-2"
        class:bg-sky-400={currentPath === item.path}
      >
        {item.title}
      </a>
    {/each}
  </nav>
</header>
<main>
  <slot />
</main>
