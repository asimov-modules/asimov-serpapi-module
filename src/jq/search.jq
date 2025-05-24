{
  "@context": {
    "know": "https://know.dev/",
    "xsd": "http://www.w3.org/2001/XMLSchema#",
    "icon": {
      "@id": "know:icon",
      "@type": "@id",
    },
    "image": {
      "@id": "know:image",
      "@type": "@id",
    },
    "items": {
      "@id": "know:items",
      "@type": "know:SearchResult",
      "@container": "@list",
    },
    "link": {
      "@id": "know:link",
      "@type": "@id",
    },
    "position": {
      "@id": "know:position",
      "@type": "xsd:integer",
    },
    "summary": {
      "@id": "know:summary",
      "@language": "en",
    },
    "title": {
      "@id": "know:title",
      "@language": "en",
    },
  },
  "@id": .search_metadata | (.bing_url // .duckduckgo_url // .google_url),
  "@type": "know:SearchResults",
  "title": (.knowledge_graph.title // .search_parameters.q),
  "summary": .knowledge_graph | (.description // .type),
  "image": .knowledge_graph | (.thumbnail // if .header_images then .header_images[0]?.image else null end),
  "items": [
    .organic_results[] | {
      "@type": "know:SearchResult",
      "position": .position,
      "title": .title,
      "summary": .snippet,
      "link": .link,
      "icon": (.favicon // .thumbnail),
    }
  ],
}
