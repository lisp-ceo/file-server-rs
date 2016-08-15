#!/bin/ruby

require 'pry'
require 'open-uri'
require 'nokogiri'

ATTR = 'Slayer'
DE = '-- '
URL = 'http://www.searchquotes.com/quotes/author/Slayer/'
FILEN = 'quotes.txt'
File.open(FILEN, File::RDWR|File::CREAT, 0644) do |file|
  file.truncate(0)
  doc = Nokogiri::HTML(open(URL))
  
  page_selector = doc.css('a.pagelink')
  pages = page_selector[page_selector.length-2].text.gsub(/\W/,"").to_i

  (2..pages).each do |p|
    if p > 2
      break
    end
    res = doc.css('a.mainquote')
    res.each{|n|
      first_letter = n.children.first.text
      rest = n.text
      if rest[0].match /[A-Z]/
        file.puts rest
      else
        file.puts [first_letter,rest].join('')      
      end
      
      file.puts [DE, ATTR].join('')
      file.puts "%"
    }
    doc = Nokogiri::HTML(open(URL + pages.to_s))
  end

end

