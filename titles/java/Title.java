package io.quicktype;

import com.fasterxml.jackson.annotation.*;

public class Title {
    private TitleElement[] titles;

    @JsonProperty("titles")
    public TitleElement[] getTitles() { return titles; }
    @JsonProperty("titles")
    public void setTitles(TitleElement[] value) { this.titles = value; }
}
