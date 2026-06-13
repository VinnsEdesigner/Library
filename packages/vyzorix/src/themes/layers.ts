export const layers = {
  card: {
    base: 'border rounded-xl p-6 relative',
    standard: 'border-white/10 bg-slate-900/65',
    subtle: 'border-white/5 bg-slate-900/40',
  },
  input: {
    base: 'w-full px-4 py-3.5 rounded-lg text-sm transition-all outline-none',
    standard: 'bg-slate-900/65 border border-white/10 text-white placeholder-slate-500 focus:border-rose-500 focus:ring-1 focus:ring-rose-500/20',
  },
  button: {
    primary: 'bg-rose-600 hover:bg-rose-500 text-white font-semibold py-4 flex items-center justify-center gap-2 text-sm tracking-wider shadow-lg shadow-rose-955/25 transition-all duration-300 rounded-xl',
    secondary: 'bg-white/5 hover:bg-white/10 border border-white/10 text-white font-semibold py-3 flex items-center justify-center gap-3 transition-colors rounded-xl text-xs',
    text: 'text-xs text-rose-500 hover:text-rose-400 font-semibold underline underline-offset-4 transition-colors',
  }
};
