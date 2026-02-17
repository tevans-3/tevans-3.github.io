#yew::prelude::*; 

#[derive(Clone, PartialEq)] 
struct Post { 
    id: u32, 
    title: String, 
    url: String, 
    content: String, //optimistically, assume that my posts have content i.e. are not 100% vacuous
}

#[derive(Clone, PartialEq)] 
struct PostListProps { 
    posts: Vec<Post>, 
}

#[function_component(App)] 
fn posts_list(PostListProps { posts }: &PostsListProps) -> Html { 
    posts.iter().map(|post| html! { 
        <p key={post.id} href= {post.url}>
        {format!({"{}", post.title)}
        <


#[derive(Clone, PartialEq)] 
struct Course { 
    catalog_number: u32,
    subject_area: String,
    title: String, 
    catalog_url: String,
}

#[derive(Properties, PartialEq)] 
struct CoursesListProps { 
    courses: Vec<Course>, 
} 

#[function_component(App)] 
fn courses_list(CoursesListProps { courses }: &CoursesListProps) -> Html { 
    courses.iter().map(|course| html! { 
        <p key={course.catalog_number} href = {course.catalog_url}>
        {format!("{} {}: {}", 
            course.subject_area, 
            course.catalog_number,
            course.title, 
        }
    }
}
            

#[function_component(App)] 
fn app() -> Html { 
	html! { 
		<h1>{ "Hi! I'm Thomas." } </h1> 
	}
	html! { 
        <div class="text">
            { format!("I'm a Database Analyst and part-time student at the University of Alberta. 
               At work, I build data analytics software. At school,  
               I study computing science and mathematics. I love building things with code, solving problems analytically, and communicating clearly (or trying to, at least :). In my free time, I enjoy reading, cycling, weightlifting, and (especially) making strange and useless things like this: {} In the past, I worked as a research assistant under the supervision of Dr. Sheny Khera; our project studied how web-form design can reduce administrative burden for academic physicians.", visualnoise.ca)
                }
            
        </div>
    }
} 

fn main() {
    println!("Hello, world!");
}
