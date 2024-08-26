<template>
  <header id="home" class="bg">
    <nav class="navbar navbar-expand-lg fixed-top dark-bg">
      <button
        class="navbar-toggler"
        type="button"
        data-toggle="collapse"
        data-target="#navbarNavAltMarkup"
        aria-controls="navbarNavAltMarkup"
        aria-expanded="false"
        aria-label="Toggle navigation"
      >
        <span class="navbar-toggler-icon"><i class="fas fa-bars" style="color:#fff; font-size:28px;"></i></span>
      </button>
      <div id="navbarNavAltMarkup" class="collapse navbar-collapse">
        <ul id="navbar-nav" class="navbar-nav mx-auto">
          <li class="active-nav">
            <a class="nav-item nav-link" href="#home">Home</a>
          </li>
          <li><a class="nav-item nav-link" href="#about">About</a></li>
          <li><a class="nav-item nav-link" href="#skills">Skills</a></li>
          <li><a class="nav-item nav-link" href="#portfolio">Portfolio</a></li>
          <li><a class="nav-item nav-link" href="#contact">Contact</a></li>
        </ul>
      </div>
    </nav>
    <Banner />
    <Arrow />
  </header>
</template>

<script>
import {invoke} from '@tauri-apps/api'
import Arrow from "@/components/Arrow.vue";
import Banner from "@/components/Banner.vue";
import data from "@/assets/data.json";

export default {
  name: "Header",
  components: {
    Banner,
    Arrow,
  },
  props: {},
  data() {
    return {
      firstname: data.main.name.first,
      lastname: data.main.name.last
    };
  },
  async created() {
      try {
          const res1 = await invoke('greet',{name:this.firstname});
          this.firstname=res1;
          const res2 = await invoke('greet',{name:this.lastname});
          this.lastname=res2;
      } catch (e) {
          console.error("{}",e);
      }
  }
};
</script>

<style lang="scss"></style>
