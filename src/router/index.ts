import { createRouter, createWebHashHistory } from "vue-router";

const routes = [
    {
        path: "/",
        component: () => import("../views/Home.vue")
    },
    {
        path: "/webSsh",
        component: () => import("../views/webssh/WebSsh.vue")
    },
    {
        name: "onlineTool",
        path: '/onlineTool/:menuKey',
        component: () => import("../views/tool/OnlineTool.vue"),
        children: [
            {
                path: 'generate',
                component: () => import("../views/tool/part/Generate.vue")
            },
            {
                path: 'json',
                component: () => import("../views/tool/part/Json.vue")
            }
        ]
    },
];

const router = createRouter({
    history: createWebHashHistory(),
    routes
});

export default router;