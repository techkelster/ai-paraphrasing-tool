import './App.css'
import TextEditor from './components/TextEditor'

function App() {
  return (
    <div className="min-h-screen bg-gradient-to-b from-slate-50 to-slate-100 py-12">
      <div className="container mx-auto px-4 max-w-4xl">
        <header className="mb-10 text-center">
          <h1 className="text-4xl font-extrabold text-slate-800 mb-3 tracking-tight">
            <span className="bg-clip-text text-transparent bg-gradient-to-r from-blue-500 to-indigo-600">
              AI Paraphrasing Tool
            </span>
          </h1>
          <p className="text-slate-600 max-w-lg mx-auto">
            Select any text to paraphrase it with state-of-the-art AI technology
          </p>
        </header>

        <main>
          <TextEditor />
        </main>

        <footer className="mt-16 text-center text-sm text-slate-500">
          <p className="flex items-center justify-center">
            <span>Powered by</span>
            <span className="ml-1 font-medium text-blue-600">Gemini AI</span>
          </p>
        </footer>
      </div>
    </div>
  )
}

export default App
