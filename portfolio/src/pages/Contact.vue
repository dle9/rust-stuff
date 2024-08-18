<template>
  <section id="contact" class="light-section">
    <div class="container-fluid">
      <h1 class="section-header">{{ heading }}</h1>
      <div class="row">
        <div
          class="col-sm-12 col-md-6 flex-col"
          data-aos="fade-right"
          data-aos-duration="1000"
          style="margin: 0 auto;"
        >
          <div class="contact-content">
            <div class="contact-text">{{ contact.text }}</div>

            <div class="contact-info">
              <ul class="list-group list-group-flush">
                <li v-if="contact.city !==''" class="list-group-item">
                  <h3 class="d-inline">
                    <i class="fas fa-map-marker-alt"></i> Location:
                  </h3>
                  <br class="d-md-none" />
                  <span>&nbsp; {{ contact.city }}</span>
                </li>
                <li class="list-group-item" v-if="contact.email !==''">
                  <h3 class="d-inline">
                    <i class="fa fa-envelope"></i> Email:
                  </h3>
                  <br class="d-md-none" />
                  <a :href="'mailto:'+ contact.email">
                    <span>{{ contact.email }}</span>
                  </a>
                </li>

                <li class="list-group-item" v-if="contact.phone !==''">
                  <h3 class="d-inline">
                    <i class="fas fa-phone"></i>  Phone:
                  </h3>
                  <br class="d-md-none" />
                  
                    <span>{{ contact.phone }}</span>
                  
                </li>
                <li class="list-group-item">
                  <h3 class="d-inline">
                    <i class="fas fa-user-plus"></i> Social:
                  </h3>
                  <br class="d-md-none" />

                  <div
                    v-for="(item, i) in social"
                    :key="i"
                    class="social-item social-spacing list-inline-item"
                  >
                    <a target="_blank" :href="item.url" :alt="item.name">
                      <i :class="'fa-lg '+item.faClass"> </i>
                    </a>
                  </div>
                </li>
                <li class="list-group-item" v-if="contact.resumeLink !==''">
                  <a :href="contact.resumeLink" target="_blank" alt="resume">
                  <button class="btn btn-resume"  target="_blank">
                    <i class="far fa-file-pdf"></i> Resume
                  </button>
                  </a>

                </li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="row arrow-container">
      <a class="arrow-icon" href="#">
        <i class="fas fa-chevron-up fa-4x"></i>
      </a>
    </div>

  </section>
</template>

<script>
import data from "@/assets/data.json";

export default {
  name: "Contact",
  props: {},
  components: {},
  data() {
    return {
      contact: data.contact,
      social: data.contact.social,
      heading: data.main.headings.contact,
    };
  },
};
</script>

<style lang="scss"></style>
