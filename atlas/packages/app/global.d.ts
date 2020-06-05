/* 

I was getting this error while running Jest:

packages/app/src/views/ExploreView/ExploreView.tsx:15:48 - error 
TS2339: Property 'flat' does not exist on type 'unknown[]'.

I tried adding "ES2019" and "ES2019.Array" to Typescript lib compiler options to no avail.
The only way I was able to fix this issue was to add 'flat' to Typescript globals (this file).

 */

interface Array<T> {
  flat(): Array<T>
  flatMap(func: (x: T) => T): Array<T>
}
