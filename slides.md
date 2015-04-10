## My Python's a Little Rusty

#### Dan Callahan &middot; dcallahan@mozilla.com &middot; @callahad

---

## The Big Lie

<br>

> "Python's fast enough, and <br>
> I can always write a C extension."

<br>

But it's not, and we never do.
<!-- .element: class="fragment" -->

---

## Why don't we write C?

### 1. Other people did it for us.

***

### Lines of C/C++ in Projects

|     Project     | # Lines |  %  |
| --------------- | -------:| ---:|
| CPython 3.5.0a2 | 399,387 | 43% |
| NumPy 1.9.2     | 166,034 | 62% |
| Pillow 2.7.0    |  22,669 | 52% |
| MarkupSafe 0.23 |     178 | 21% |

***

### We're standing on their shoulders

### And so are they.
<!-- .element: class="fragment" -->

![Pillow links to libjpeg](img/libjpeg.png)
<!-- .element: class="fragment" style="max-height: 65%; max-width: 65%;" -->

***

### 2. Writing C is **scary**.

***

### Memory management is **hard**.

Much like the video at

[youtube.com/watch?v=718fskG2n34](https://www.youtube.com/watch?v=718fskG2n34)

C has no safety belts.

<!-- <video data-autoplay class="stretch" src="img/ghostride.mp4"></video> -->

***

- Heartbleed
- Ghost <!-- .element: class="fragment" -->
- CVE-2015-0080 <!-- .element: class="fragment" -->

_I'm not smarter than the glibc or openssl devs._
<!-- .element: class="fragment" -->

---

## But what if you **need** to?

***

### The Dream

<br>

<span class="fragment">C's Performance</span><span class="fragment">, Portability</span><span class="fragment">, and Embeddability.</span>

<!-- .element: class="fragment" --> With *guaranteed* safety.

---

## Rust.

---

## Stack vs. Heap

***

|      Stack      |     Heap      |
| --------------- | ------------- |
| Fast but tiny   | Slow but huge |
| Function locals | Globals       |
| Managed by CPU  | Unmanaged     |

<br>

_Only small values of known, fixed size can go on the stack.
<br>
Growable things like vectors must go on the heap._

---

## Managed vs unmanaged

***
<!-- .slide: data-background-transition="none" data-background="img/ownership/01.jpg" -->

&nbsp;

***
<!-- .slide: data-background-transition="none" data-background="img/ownership/02.jpg" -->

&nbsp;

***
<!-- .slide: data-background-transition="none" data-background="img/ownership/03.jpg" -->

&nbsp;

***
<!-- .slide: data-background-transition="none" data-background="img/ownership/04.jpg" -->

&nbsp;


---

## Ownership

<!-- .slide: data-background-transition="none" -->

***
<!-- .slide: data-background-transition="none" data-background="img/ownership/05.jpg" -->

&nbsp;

***
<!-- .slide: data-background-transition="none" data-background="img/ownership/06.jpg" -->

&nbsp;

***
<!-- .slide: data-background-transition="none" data-background="img/ownership/07.jpg" -->

&nbsp;

***
<!-- .slide: data-background-transition="none" data-background="img/ownership/08.jpg" -->

&nbsp;

***
<!-- .slide: data-background-transition="none" data-background="img/ownership/09.jpg" -->

&nbsp;

***
<!-- .slide: data-background-transition="none" data-background="img/ownership/10.jpg" -->

&nbsp;

***
<!-- .slide: data-background-transition="none" data-background="img/ownership/11.jpg" -->

&nbsp;

***
<!-- .slide: data-background-transition="none" data-background="img/ownership/12.jpg" -->

&nbsp;

***
<!-- .slide: data-background-transition="none" data-background="img/ownership/13.jpg" -->

&nbsp;

***
<!-- .slide: data-background-transition="none" data-background="img/ownership/14.jpg" -->

&nbsp;

***
<!-- .slide: data-background-transition="none" data-background="img/ownership/15.jpg" -->

&nbsp;

***
<!-- .slide: data-background-transition="none" data-background="img/ownership/16.jpg" -->

&nbsp;

***
<!-- .slide: data-background-transition="none" data-background="img/ownership/17.jpg" -->

&nbsp;

***
<!-- .slide: data-background-transition="none" data-background="img/ownership/18.jpg" -->

&nbsp;

***
<!-- .slide: data-background-transition="none" data-background="img/ownership/19.jpg" -->

&nbsp;

***
<!-- .slide: data-background-transition="none" data-background="img/ownership/20.jpg" -->

&nbsp;

***

### Enforced Statically at Compile Time

<br>

- No dangling pointers
- No use after free vulnerabilities
- No pointer arithmetic
- No null pointer dereferencing

<br>

_This is a "Zero-Cost Abstraction."_

---

## Mutability Demos!

---

![](img/servo-github.png)
<!-- .element: style="margin-top: -5%;" -->

---

![](img/servo-reddit.png)
<!-- .element: style="margin-top: -5%;" -->

---

## Python FFI Demos!

---

## Learn More

_rust-lang.org_

---

## Questions?

<br>

@callahad

dcallahan@mozilla.com

github.com/callahad/pycon2015-rust

___

rust-lang.org
