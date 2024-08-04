---
title: 'Setting up a blog with Nuxt and Nuxt Content'
description: 'An epic journey all for the sake of writing blog posts in markdown'
date: '2023-01-30'
---

This is my first post using Nuxt Content. I have been using Nuxt for my personal website for a while, but I decided it was time to start a blog where I can write about my projects and other things I find interesting.

<!-- more -->

## [Research]

The first step I took after deciding to start a blog was to find the best way to write my posts. The first thing that came to mind was to just make a custom vue component/page for each post. This would allow me to have full control over the styling and layout of each post. However, I felt that the time overhead that would introduce would lead to more time being spent writing vue components, which would be better spent writing posts.

After some research, I found [Nuxt Content](https://content.nuxtjs.org). This is a module for Nuxt that allows you to write your posts in markdown and have them automatically get rendered in a vue component. This is perfect for me, as I can spend time making the blog look nice one time, and then easily write posts in markdown. This method also allows me to easily add the posts to git and keep the blog content with the code, rather than having them stored somewhere else, like in a database.

## [Installation]

The first step to setting up Content is to install it. Since I am adding the blog an existing Nuxt project, I followed the instructions for adding it to an existing project. I'll walk through what I did to set up my blog, but you can always find the latest instructions on the Content [getting started](https://content.nuxtjs.org/get-started) page.

First, I installed the `@nuxt/content` package using pnpm. If you are using npm, you can replace `pnpm` with `npm` in the command below.

```bash
pnpm add @nuxt/content
```

The next step is to add the module to the `nuxt.config.ts` or `nuxt.config.js` file. This is done by adding `@nuxt/content` to the `modules` array.

```ts
export default {
  ...
  modules: [
    '@nuxt/content'
  ]
  ...
}
```

## [Setup]

Now you are ready to set up the blog. This is done by creating a new directory in the project root called `content/blog`, where all of your blog post's markdown will be stored. You can add as many subdirectories as you want to organize your posts. For testing purposes, I added a file called `first-post.md` to the `content/blog` directory.

```md
---
title: 'My First Post'
description: 'This is my first post using Nuxt Content'
date: '2023-01-30'
---

This is some content for my first post.
```

The `---` at the top of the file is called the front matter, which is used to store metadata about the post. The `title` and `description` fields are used to set the title and description of the page, which is used by search engines and other services. The `date` field is used to set the date of the post, which can be used to sort the posts on the blog page. You can see more information about the front matter in the Nuxt Content [documentation](https://content.nuxtjs.org/guide/writing/markdown#front-matter).

The next step is to create a new page in the `pages` directory called `blog/[...slug].vue` which is the page that will be used to display each post. Using the `...slug` in the file name tells Nuxt to match any URL that starts with `/blog/`. The `slug` is the name of the file without the extension, so the URL for the `first-post.md` file would be `/blog/first-post`. You can add the following code to the `blog/[...slug].vue` file to display the post.

```vue
<template>
  <main>
    <h1>{{ page.title }}</h1>
    <p>{{ page.description }}</p>
    <ContentDoc />
  </main>
</template>

<script lang="ts" setup>
const { page } = useContent()
</script>
```

The `page` variable is automatically created by the `useContent` function and contains the front matter of the post. The `ContentDoc` component is a component that will automatically render the markdown content of the post based on the `slug` variable (URL).

Now, if you navigate to `/blog/first-post`, you should see the post you created. If you want to see the list of posts when you navigate to `/blog`, you can add the following code to the `blog/index.vue` file.

```vue
<template>
  <div>
    <h1>Posts</h1>
    <ContentList v-slot="{ list }" path="/blog" :query="{ sort: { date: -1 } }">
      <ul>
        <li v-for="post in list" :key="post._slug">
          <NuxtLink :to="post._path">{{ post.title }}</NuxtLink>
        </li>
      </ul>
    </ContentList>
  </div>
</template>
```

The `ContentList` component will automatically create a list of all the posts in the `content/blog` directory. The `:query="{ sort: { date: -1 } }"` attribute sorts the posts in reverse order based on their date, with the newest posts at the top of the list. You can then use the `list` variable to display the posts in any way you want.

## [What's Next]

Now that I have a basic blog setup, I can start writing posts. I am excited to start writing about my projects and other things I find interesting. I need to spend some time making the blog look nice, but I think it is a good start.

If you are reading this far in the future, you will see whatever the most up-to-date styling is, so just imagine that it looked bad when I wrote this. Hopefully it looks decent now!

The source code for this blog and my whole personal website is available on [GitHub](https://github.com/SquarePear/www). If you have any questions or suggestions, feel free to open an issue or pull request!

## [Final Thoughts]

Nuxt Content is a great module for Nuxt.js that allows you to write your blog posts in markdown and have them automatically get rendered in a vue component. I am excited to be able to quickly write posts about my projects and any interesting issues I encounter while working on them. I am also excited to be able to keep the blog content with the code, rather than having it stored somewhere else like in a database.
