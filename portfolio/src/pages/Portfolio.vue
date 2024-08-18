<template>
  <section id="portfolio" class="dark-section">
    <div class="container-fluid">
      <h1 class="section-header">{{ heading }}</h1>

      <div id="portfolio-container" class="row">
        <div
          data-aos="fade-right"
          data-aos-duration="1000"
          v-for="(item, index) in projects"
          :key="index"
          class="col-sm-12 col-md-4 flex-col"
        >
          <div class="portfolio-item">
            <img :src="require(`@/assets/images/portfolio/${item.image}`)" />
            <div class="overlay">
              <a :href="item.link" title="View project" target="_blank">
                <div class="portfolio-item-content">
                  <h3>{{ item.title }}</h3>
                  <p>{{ item.description }}</p>
                </div>
              </a>
            </div>
            <h2>{{item.title}}</h2>
          </div>
        </div>
      </div>
    </div>
    <Arrow />
  </section>
</template>

<script>
import data from "@/assets/data.json";
import Arrow from "@/components/Arrow.vue";

export default {
  name: "portfolio",
  props: {},
  components: {
    Arrow,
  },
  data() {
    return {
      projects: data.portfolio.projects,
      heading: data.main.headings.portfolio,
      currentFilter: data.portfolio.defaultFilter,
    };
  },
  computed: {
    filteredProjects() {
      var projects = data.portfolio.projects;
      var filter = this.currentFilter;
      var filtered = projects.filter(function(x) {
        return x.filter === filter;
      });
      return filtered;
    },
    filters() {
      var filterList = [];
      var projects = data.portfolio.projects;
      filterList = projects.filter(function(x) {
        if (!filterList.includes(x.filter)) {
          filterList.push(x.filter);
          return x.filter;
        }
      });
      return filterList;
    },
  },
  methods: {
    setFilter(event) {
      this.currentFilter = event.target.dataset.filter;
    },
  },
};
</script>

<style lang="scss"></style>
