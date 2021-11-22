# Function declaration keyword
This documents the reasoning for choosing `fn` as the function declaration keyword over `function`.


## The case for `function`
Since this language is inspired by TypeScript/JavaScript, it might seem obvious to choose `function` as the keyword over `fn`.


## The case for `fn`
At first explicit syntaxes were preferred, however although `function` is more explicit, it conflicted with the language goal of being easier to read. And since the preference is to use lesser code to deliver the same effect and **clarity** fn makes code less "wordy" so that developers can focus on the actual logic rather than the noise of boilerplate.

The idea is that things like variable lifetime / definitions should be made explicit, but that does not mean that the words for making it explicit should be long, because we want to make the code's meaning explicit, not the wording explicit and long winded!